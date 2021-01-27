//! Activity
use crate::bpmn::schema::{
    ActivityType, Expr, FlowNodeType, FormalExpression, LoopCharacteristics,
    MultiInstanceLoopCharacteristics, SequenceFlow, StandardLoopCharacteristics,
};
use crate::data_object;
use crate::flow_node::{self, Action, FlowNode, IncomingIndex, OutgoingIndex, StateError};
use crate::language::{Engine as _, MultiLanguageEngine};
use crate::process::{self, Log};
use factory::ParameterizedFactory;
use futures::stream::{Stream, StreamExt};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use streamunordered::{StreamUnordered, StreamYield};
use tokio::sync::{broadcast, oneshot, watch};
use tokio::task;

pub mod script_task;

pub trait Activity: FlowNode {
    /// Signals execution request
    fn execute(&mut self);
}

pub struct ActivityContainer<T, E, F>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin,
{
    state: InnerState,
    variant: Variant<T>,
    flow_nodes: StreamUnordered<T>,
    flow_node_tokens: Vec<usize>,
    element: E,
    engine: Arc<MultiLanguageEngine>,
    notifier: broadcast::Sender<Completion>,
    notifier_receiver: broadcast::Receiver<Completion>,
    log_broadcast: Option<broadcast::Sender<Log>>,
    waker: Option<Waker>,
    waker_sender: watch::Sender<Option<Waker>>,
    waker_receiver: watch::Receiver<Option<Waker>>,
    flow_node_maker: F,
    process: Option<process::Handle>,
}

#[derive(Debug)]
enum Variant<T> {
    Initializing {
        activities: oneshot::Receiver<(IncomingIndex, Vec<T>)>,
    },
    Initialized,
    Ready {
        // for standard loop
        test_before: bool,
    },
    Executing,
    Testing,
    Errored,
    Done,
}

#[derive(Clone, Debug)]
enum Completion {
    Success(bool),
    Error,
}

impl<T, E, F> ActivityContainer<T, E, F>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin,
{
    pub fn new(element: E, flow_node_maker: F) -> Self {
        let (notifier, notifier_receiver) = broadcast::channel(1);
        let (waker_sender, waker_receiver) = watch::channel(None);
        let mut activities = match element.loop_characteristics() {
            None => vec![flow_node_maker.create(element.clone())],
            Some(LoopCharacteristics::StandardLoopCharacteristics(_)) => {
                vec![flow_node_maker.create(element.clone())]
            }
            Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(_)) => {
                // Can't prepare multi-instance activities at creation
                vec![]
            }
        };

        let mut flow_node_tokens = vec![];
        let mut flow_nodes = StreamUnordered::new();
        for activity in activities.drain(..) {
            flow_node_tokens.push(flow_nodes.push(activity));
        }

        Self {
            state: InnerState {
                counter: BigInt::from(0usize),
            },
            variant: Variant::Initialized,
            flow_nodes,
            flow_node_tokens,
            element,
            engine: Arc::new(MultiLanguageEngine::new()),
            notifier,
            notifier_receiver,
            log_broadcast: None,
            waker: None,
            waker_sender,
            waker_receiver,
            flow_node_maker,
            process: None,
        }
    }

    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn wake_when_ready(&mut self, waker: Waker) {
        let _ = self.waker_sender.send(Some(waker.clone()));
        self.waker.replace(waker);
    }
}

/// Activity State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State(Vec<flow_node::State>, InnerState);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InnerState {
    counter: BigInt,
}

impl<T, E, F> FlowNode for ActivityContainer<T, E, F>
where
    T: Activity + 'static,
    E: ActivityType + Clone + Unpin,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin + 'static,
{
    fn set_state(&mut self, state: flow_node::State) -> Result<(), StateError> {
        match state {
            flow_node::State::ActivityState(mut state) => {
                if state.0.len() != self.flow_nodes.len() {
                    return Err(StateError::InvalidVariant);
                }
                for (inner_state, flow_node) in state.0.drain(..).zip(self.flow_nodes.iter_mut()) {
                    flow_node.set_state(inner_state)?;
                }

                self.state = state.1;
            }
            _ => return Err(StateError::InvalidVariant),
        }
        self.wake();
        Ok(())
    }

    fn get_state(&mut self) -> flow_node::State {
        flow_node::State::ActivityState(State(
            self.flow_nodes.iter_mut().map(|f| f.get_state()).collect(),
            self.state.clone(),
        ))
    }

    fn set_process(&mut self, process: process::Handle) {
        self.engine = Arc::new(process.model().script_engine_factory().create());
        self.log_broadcast.replace(process.log_broadcast());
        for flow_node in self.flow_nodes.iter_mut() {
            flow_node.set_process(process.clone());
        }
        self.process.replace(process);
        self.wake();
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.clone())
    }

    fn sequence_flow(
        &mut self,
        outgoing: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
        for flow_node in self.flow_nodes.iter_mut() {
            flow_node.sequence_flow(outgoing, sequence_flow, condition_result);
        }
        self.wake();
    }

    fn handle_outgoing_action(
        &mut self,
        index: OutgoingIndex,
        action: Option<Action>,
    ) -> Option<Option<Action>> {
        self.flow_nodes
            .iter_mut()
            .fold(Some(action), |action, flow_node| match action {
                None => None,
                Some(action) => flow_node.handle_outgoing_action(index, action),
            })
    }

    fn incoming(&mut self, index: IncomingIndex) {
        if let Variant::Initialized = self.variant {
            // If we need to launch a multi-instance activity, we need to populate the flow nodes
            // upon activation
            if self.flow_nodes.is_empty() {
                if let Some(process) = self.process.clone() {
                    let (sender, receiver) = oneshot::channel();
                    let element = self.element.clone();
                    let flow_node_maker = self.flow_node_maker.clone();
                    let waker_receiver = self.waker_receiver.clone();
                    task::spawn(async move {
                        let loop_characteristics = element.loop_characteristics();
                        let activities = match loop_characteristics {
                            // Multi-instance loops:
                            //  - loopCardinality given
                            Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(
                                MultiInstanceLoopCharacteristics {
                                    loop_cardinality: Some(Expr::FormalExpression(expr)),
                                    ..
                                },
                            )) => {
                                let engine = process.model().expression_engine_factory().create();
                                match engine.eval::<usize>(expr).await {
                                    Ok(cardinality) => (0..cardinality)
                                        .into_iter()
                                        .map(|_| flow_node_maker.create(element.clone()))
                                        .collect(),
                                    Err(err) => {
                                        let _ =
                                            process.log_broadcast().send(Log::ExpressionError {
                                                error: format!("{:?}", err),
                                            });
                                        vec![]
                                    }
                                }
                            }
                            // - loopDataInput given
                            Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(
                                MultiInstanceLoopCharacteristics {
                                    loop_data_input_ref: Some(loop_data_input),
                                    ..
                                },
                            )) => {
                                match process.data_object(loop_data_input).await {
                                    Ok(data_object) => {
                                        let data_object = data_object.read().await;
                                        if let Some(collection) =
                                            data_object.downcast_ref::<data_object::Collection>()
                                        {
                                            (0..collection.len())
                                                .into_iter()
                                                .map(|_| flow_node_maker.create(element.clone()))
                                                .collect()
                                        } else {
                                            // it is not a collection
                                            // TODO: handle error
                                            vec![]
                                        }
                                    }
                                    Err(_) => {
                                        // there is no such data object
                                        // TODO: handle error
                                        vec![]
                                    }
                                }
                            }
                            // otherwise, do nothing
                            _ => vec![],
                        };

                        let _ = sender.send((index, activities));
                        let waker = waker_receiver.borrow();
                        if let Some(waker) = waker.as_ref() {
                            waker.wake_by_ref();
                        }
                    });
                    self.variant = Variant::Initializing {
                        activities: receiver,
                    };
                    self.wake();
                    return;
                }
            }

            self.variant = Variant::Ready { test_before: false };
        }
        for flow_node in self.flow_nodes.iter_mut() {
            flow_node.incoming(index);
        }
        self.wake();
    }

    fn tokens(&mut self, count: usize) {
        for flow_node in self.flow_nodes.iter_mut() {
            flow_node.tokens(count)
        }
        self.wake();
    }
}

impl<T, E, F> ActivityContainer<T, E, F>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin,
{
    fn spawn_test(&self, expression: FormalExpression) {
        let engine = self.engine.clone();
        let notifier = self.notifier.clone();
        let log_broadcast = self.log_broadcast.clone();
        let waker_receiver = self.waker_receiver.clone();
        task::spawn(async move {
            let result = engine.eval(&expression).await;
            // we're holding it until the end of the block
            // so that a new write won't start until a read lock
            // has been released
            let waker = waker_receiver.borrow();
            match result {
                Ok(val) => {
                    let _ = notifier.send(Completion::Success(val));
                }
                Err(err) => {
                    let _ = notifier.send(Completion::Error);
                    if let Some(ref log_broadcast) = log_broadcast {
                        let _ = log_broadcast.send(Log::ScriptError {
                            error: format!("{:?}", err),
                        });
                    }
                }
            }
            if let Some(waker) = waker.as_ref() {
                waker.wake_by_ref();
            }
        });
    }

    fn handle_flow_node(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Action>> {
        match self.element.loop_characteristics() {
            None => {
                let token = self.flow_node_tokens[0];
                if !matches!(self.variant, Variant::Executing) {
                    if let Some(flow_node) = self.flow_nodes.get_mut(token) {
                        flow_node.execute()
                    }
                    self.variant = Variant::Executing;
                }
                self.wake_when_ready(cx.waker().clone());
                self.flow_nodes.poll_next_unpin(cx).map(|item| match item {
                    Some((StreamYield::Item(item), _)) => Some(item),
                    Some((StreamYield::Finished(stream), _)) => {
                        stream.remove(Pin::new(&mut self.flow_nodes));
                        None
                    }
                    None => None,
                })
            }
            // Standard loop
            //   - with loopCondition specified
            Some(LoopCharacteristics::StandardLoopCharacteristics(
                StandardLoopCharacteristics {
                    test_before,
                    loop_maximum,
                    loop_condition: Some(Expr::FormalExpression(expression)),
                    ..
                },
            )) => {
                if let Some(max) = loop_maximum {
                    if max == &self.state.counter {
                        self.variant = Variant::Initialized;
                        self.wake_when_ready(cx.waker().clone());
                        return Poll::Pending;
                    }
                }
                let test_before = *test_before;
                let should_run = !matches!(test_before, Some(true))
                    || matches!(self.variant, Variant::Ready { test_before: true });
                let expression = expression.clone();
                let me = self.get_mut();
                if should_run {
                    if let Variant::Ready { .. } = me.variant {
                        if let Some(flow_node) = me.flow_nodes.get_mut(me.flow_node_tokens[0]) {
                            flow_node.execute()
                        }
                        me.variant = Variant::Executing;
                    }
                    let result = me.flow_nodes.poll_next_unpin(cx);
                    match result {
                        Poll::Pending | Poll::Ready(None) => {
                            me.wake_when_ready(cx.waker().clone());
                            Poll::Pending
                        }
                        Poll::Ready(Some(item)) => match item {
                            (StreamYield::Item(action), _) => {
                                me.state.counter += 1;
                                if !matches!(test_before, Some(true)) {
                                    me.variant = Variant::Testing;
                                    me.spawn_test(expression);
                                }
                                Poll::Ready(Some(action))
                            }
                            (StreamYield::Finished(finished), _) => {
                                finished.remove(Pin::new(&mut me.flow_nodes));
                                me.wake_when_ready(cx.waker().clone());
                                Poll::Pending
                            }
                        },
                    }
                } else {
                    me.variant = Variant::Testing;
                    me.wake_when_ready(cx.waker().clone());
                    me.spawn_test(expression);
                    Poll::Pending
                }
            }
            //  - with no loopCondition specified
            Some(LoopCharacteristics::StandardLoopCharacteristics(
                StandardLoopCharacteristics { .. },
            )) => {
                // From the specification:
                // The looping behavior MAY be underspecified, meaning that the modeler can simply
                // document the condition, in which case the loop cannot be formally executed.
                self.wake_when_ready(cx.waker().clone());
                Poll::Pending
            }
            // MultiInstanceLoopCharacteristics
            //  - no instances left
            Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(_))
                if self.flow_nodes.is_empty() =>
            {
                self.wake_when_ready(cx.waker().clone());
                Poll::Pending
            }
            //  - sequential
            Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(
                MultiInstanceLoopCharacteristics {
                    is_sequential: Some(true),
                    ..
                },
            )) => {
                let me = self.get_mut();
                if let Variant::Ready { .. } = me.variant {
                    if let Some(flow_node) = me.flow_nodes.get_mut(me.flow_node_tokens[0]) {
                        flow_node.execute()
                    }
                    me.variant = Variant::Executing;
                }
                let result = me.flow_nodes.poll_next_unpin(cx);
                match result {
                    Poll::Pending => {
                        me.wake_when_ready(cx.waker().clone());
                        Poll::Pending
                    }
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Ready(Some(item)) => match item {
                        (StreamYield::Item(action), token) => {
                            me.state.counter += 1;
                            // unschedule the instance
                            me.remove_token(token, true);

                            me.variant = Variant::Ready { test_before: false };
                            Poll::Ready(Some(action))
                        }
                        (StreamYield::Finished(finished), token) => {
                            finished.remove(Pin::new(&mut me.flow_nodes));
                            me.remove_token(token, false);
                            me.wake_when_ready(cx.waker().clone());
                            Poll::Pending
                        }
                    },
                }
            }
            //  - parallel
            Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(
                MultiInstanceLoopCharacteristics { .. },
            )) => {
                let me = self.get_mut();
                if let Variant::Ready { .. } = me.variant {
                    for flow_node in me.flow_nodes.iter_mut() {
                        flow_node.execute()
                    }

                    me.variant = Variant::Executing;
                }
                let result = me.flow_nodes.poll_next_unpin(cx);
                match result {
                    Poll::Pending => {
                        me.wake_when_ready(cx.waker().clone());
                        Poll::Pending
                    }
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Ready(Some(item)) => match item {
                        (StreamYield::Item(action), token) => {
                            me.state.counter += 1;
                            // unschedule the instance
                            me.remove_token(token, true);

                            if me.flow_node_tokens.is_empty() {
                                me.variant = Variant::Ready { test_before: false };
                            }
                            Poll::Ready(Some(action))
                        }
                        (StreamYield::Finished(finished), token) => {
                            finished.remove(Pin::new(&mut me.flow_nodes));
                            me.remove_token(token, false);
                            me.wake_when_ready(cx.waker().clone());
                            Poll::Pending
                        }
                    },
                }
            }
        }
    }

    fn remove_token(&mut self, token: usize, from_stream: bool) {
        if from_stream {
            let pinned = Pin::new(&mut self.flow_nodes);
            pinned.remove(token);
        }
        if let Some((index, _)) = self
            .flow_node_tokens
            .iter()
            .enumerate()
            .find(|(_, token_)| **token_ == token)
        {
            self.flow_node_tokens.remove(index);
        }
    }
}

impl<T, E, F> Stream for ActivityContainer<T, E, F>
where
    T: Activity + 'static,
    E: ActivityType + Unpin + Clone,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin + 'static,
{
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.variant {
            Variant::Initializing {
                ref mut activities, ..
            } => match activities.try_recv() {
                Ok((index, mut activities)) => {
                    let me = self.get_mut();
                    for mut activity in activities.drain(..) {
                        // Set up activity:
                        // 1. Process
                        if let Some(process) = me.process.clone() {
                            activity.set_process(process);
                        }
                        // 2. Can't restore their state as the number of instances and their data may vary
                        // 3. Signal an incoming (since Initializing can only get started this way)
                        activity.incoming(index);
                        me.flow_node_tokens.push(me.flow_nodes.push(activity));
                    }
                    me.variant = Variant::Initialized;
                    me.incoming(index);
                    me.poll_next_unpin(cx)
                }
                Err(oneshot::error::TryRecvError::Empty) => {
                    self.wake_when_ready(cx.waker().clone());
                    Poll::Pending
                }
                Err(_) => {
                    // TODO: handle this error
                    self.wake_when_ready(cx.waker().clone());
                    Poll::Pending
                }
            },
            Variant::Initialized => {
                self.wake_when_ready(cx.waker().clone());
                Poll::Pending
            }
            Variant::Ready { .. } | Variant::Executing => self.handle_flow_node(cx),
            Variant::Testing => match self.notifier_receiver.try_recv() {
                // should continue
                Ok(Completion::Success(true)) => {
                    self.variant = Variant::Ready { test_before: true };
                    self.handle_flow_node(cx)
                }
                // should not continue
                Ok(Completion::Success(false)) => {
                    self.variant = Variant::Done;
                    self.wake_when_ready(cx.waker().clone());
                    Poll::Pending
                }
                Ok(Completion::Error) => {
                    self.variant = Variant::Errored;
                    Poll::Ready(Some(Action::Complete))
                }
                Err(broadcast::error::TryRecvError::Empty) => {
                    self.wake_when_ready(cx.waker().clone());
                    Poll::Pending
                }
                Err(broadcast::error::TryRecvError::Lagged(_)) => {
                    self.wake_when_ready(cx.waker().clone());
                    Poll::Pending
                }
                Err(broadcast::error::TryRecvError::Closed) => Poll::Ready(None),
            },
            Variant::Errored => {
                self.variant = Variant::Ready { test_before: false };
                self.wake_when_ready(cx.waker().clone());
                Poll::Pending
            }
            Variant::Done => {
                self.variant = Variant::Ready { test_before: false };
                self.wake_when_ready(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::bpmn::parse;
    use crate::data_object;
    use crate::language::*;
    use crate::model;
    use crate::test::*;
    use std::sync::Arc;
    use std::sync::Mutex;
    use tokio::sync::{broadcast, mpsc};

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn standard_loop_after() {
        let definitions = parse(include_str!("test_models/standard_loop.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let (ctrl_sender, _) = broadcast::channel(10);
        let ctrl_sender_clone = ctrl_sender.clone();
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn(
                    "notify",
                    move || {
                        while let Err(_) = sender.try_send(()) {}
                    },
                );
                let ctrl_receiver = Arc::new(Mutex::new(ctrl_sender_clone.subscribe()));
                rhai_engine.register_fn("should_run", move || {
                    let mut ctrl_receiver = ctrl_receiver.lock().unwrap();
                    loop {
                        match ctrl_receiver.try_recv() {
                            Err(broadcast::error::TryRecvError::Empty) => {}
                            Err(broadcast::error::TryRecvError::Lagged(_)) => {}
                            Err(_) => return false,
                            Ok(v) => return v,
                        }
                    }
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        assert!(handle.start().await.is_ok());

        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        let _ = ctrl_sender.send(true);
        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        let _ = ctrl_sender.send(false);

        // It should stop when `should_run()` is false
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn standard_loop_test_before() {
        let definitions =
            parse(include_str!("test_models/standard_loop_test_before.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let (ctrl_sender, _) = broadcast::channel(10);
        let ctrl_sender_clone = ctrl_sender.clone();
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn(
                    "notify",
                    move || {
                        while let Err(_) = sender.try_send(()) {}
                    },
                );
                let ctrl_receiver = Arc::new(Mutex::new(ctrl_sender_clone.subscribe()));
                rhai_engine.register_fn("should_run", move || {
                    let mut ctrl_receiver = ctrl_receiver.lock().unwrap();
                    loop {
                        match ctrl_receiver.try_recv() {
                            Err(broadcast::error::TryRecvError::Empty) => {}
                            Err(broadcast::error::TryRecvError::Lagged(_)) => {}
                            Err(_) => return false,
                            Ok(v) => return v,
                        }
                    }
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        assert!(handle.start().await.is_ok());

        // Until we allow the activity to proceed, nothing should happen
        assert!(timeout(receiver.recv()).await.is_err());

        let _ = ctrl_sender.send(true);
        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        let _ = ctrl_sender.send(true);

        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        let _ = ctrl_sender.send(false);

        // It should stop when `should_run()` is false
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn standard_loop_max() {
        let definitions = parse(include_str!("test_models/standard_loop_max.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let (ctrl_sender, _) = broadcast::channel(10);
        let ctrl_sender_clone = ctrl_sender.clone();
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn(
                    "notify",
                    move || {
                        while let Err(_) = sender.try_send(()) {}
                    },
                );
                let ctrl_receiver = Arc::new(Mutex::new(ctrl_sender_clone.subscribe()));
                rhai_engine.register_fn("should_run", move || {
                    let mut ctrl_receiver = ctrl_receiver.lock().unwrap();
                    loop {
                        match ctrl_receiver.try_recv() {
                            Err(broadcast::error::TryRecvError::Empty) => {}
                            Err(broadcast::error::TryRecvError::Lagged(_)) => {}
                            Err(_) => return false,
                            Ok(v) => return v,
                        }
                    }
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        assert!(handle.start().await.is_ok());

        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        let _ = ctrl_sender.send(true);
        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        let _ = ctrl_sender.send(true);

        // It should stop when `should_run()` is false or maximum cap (2) has been reached
        // (and it has)
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn multi_loop_cardinality() {
        let definitions = parse(include_str!("test_models/multi_loop_cardinality.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn(
                    "notify",
                    move || {
                        while let Err(_) = sender.try_send(()) {}
                    },
                );
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        assert!(handle.start().await.is_ok());

        for _ in 0..3 {
            // should repeat 3 times
            assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        }

        // It should stop when cardinality is exhausted
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn multi_loop_cardinality_parallel() {
        let definitions = parse(include_str!(
            "test_models/multi_loop_cardinality_parallel.bpmn"
        ))
        .unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        use std::sync::Barrier;
        let barrier = Arc::new(Barrier::new(3));
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                let barrier = Arc::clone(&barrier);
                rhai_engine.register_fn("notify", move || {
                    barrier.wait();
                    while let Err(_) = sender.try_send(()) {}
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        assert!(handle.start().await.is_ok());

        for _ in 0..3 {
            // should repeat 3 times
            assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        }

        // It should stop when cardinality is exhausted
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn multi_loop_data_object() {
        let definitions = parse(include_str!("test_models/multi_loop_data_object.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn(
                    "notify",
                    move || {
                        while let Err(_) = sender.try_send(()) {}
                    },
                );
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        {
            let data_object = handle.data_object("data_object").await.unwrap();

            let mut write = data_object.write().await;
            *write = Box::new(data_object::Collection(vec![
                Box::new(data_object::Empty),
                Box::new(data_object::Empty),
                Box::new(data_object::Empty),
            ]));
        }

        assert!(handle.start().await.is_ok());

        for _ in 0..3 {
            // should repeat 3 times
            assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        }

        // It should stop when cardinality is exhausted
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn multi_loop_data_object_parallel() {
        let definitions = parse(include_str!(
            "test_models/multi_loop_data_object_parallel.bpmn"
        ))
        .unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        use std::sync::Barrier;
        let barrier = Arc::new(Barrier::new(3));
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                let barrier = Arc::clone(&barrier);
                rhai_engine.register_fn("notify", move || {
                    barrier.wait();
                    while let Err(_) = sender.try_send(()) {}
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        {
            let data_object = handle.data_object("data_object").await.unwrap();

            let mut write = data_object.write().await;
            *write = Box::new(data_object::Collection(vec![
                Box::new(data_object::Empty),
                Box::new(data_object::Empty),
                Box::new(data_object::Empty),
            ]));
        }

        assert!(handle.start().await.is_ok());

        for _ in 0..3 {
            // should repeat 3 times
            assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(()));
        }

        // It should stop when cardinality is exhausted
        assert!(timeout(receiver.recv()).await.is_err());
    }
}
