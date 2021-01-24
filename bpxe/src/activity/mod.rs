//! Activity
use crate::bpmn::schema::{
    ActivityType, Expr, FlowNodeType, FormalExpression, LoopCharacteristics, SequenceFlow,
    StandardLoopCharacteristics,
};
use crate::flow_node::{self, Action, FlowNode, IncomingIndex, OutgoingIndex, StateError};
use crate::language::{Engine as _, MultiLanguageEngine};
use crate::process::{self, Log};
use futures::stream::{Stream, StreamExt};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::{broadcast, watch};
use tokio::task;

pub mod script_task;

pub trait Activity: FlowNode {
    // Signals execution request
    fn execute(&mut self);
}

pub struct ActivityContainer<T, E>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
{
    state: InnerState,
    variant: Variant,
    flow_nodes: Vec<T>,
    element: E,
    engine: Arc<MultiLanguageEngine>,
    notifier: broadcast::Sender<Completion>,
    notifier_receiver: broadcast::Receiver<Completion>,
    log_broadcast: Option<broadcast::Sender<Log>>,
    waker: Option<Waker>,
    waker_sender: watch::Sender<Option<Waker>>,
    waker_receiver: watch::Receiver<Option<Waker>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Variant {
    Initialized,
    Ready { test_before: bool },
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

impl<T, E> ActivityContainer<T, E>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
{
    pub fn new<F>(element: E, flow_node_maker: F) -> Self
    where
        F: Fn(E) -> T,
    {
        let (notifier, notifier_receiver) = broadcast::channel(1);
        let (waker_sender, waker_receiver) = watch::channel(None);
        let flow_nodes = match element.loop_characteristics() {
            None => vec![flow_node_maker(element.clone())],
            Some(LoopCharacteristics::StandardLoopCharacteristics(_)) => {
                vec![flow_node_maker(element.clone())]
            }
            // TODO: implement `MultiInstanceLoopCharacteristics`
            _ => vec![flow_node_maker(element.clone())],
        };
        Self {
            state: InnerState {
                counter: BigInt::from(0usize),
            },
            variant: Variant::Initialized,
            flow_nodes,
            element,
            engine: Arc::new(MultiLanguageEngine::new()),
            notifier,
            notifier_receiver,
            log_broadcast: None,
            waker: None,
            waker_sender,
            waker_receiver,
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

impl<T, E> FlowNode for ActivityContainer<T, E>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
{
    fn set_state(&mut self, state: flow_node::State) -> Result<(), StateError> {
        match state {
            flow_node::State::ActivityState(mut state) => {
                if state.0.len() != self.flow_nodes.len() {
                    return Err(StateError::InvalidVariant);
                }
                for (index, inner_state) in state.0.drain(..).enumerate() {
                    self.flow_nodes[index].set_state(inner_state)?;
                }

                self.state = state.1;
            }
            _ => return Err(StateError::InvalidVariant),
        }
        self.wake();
        Ok(())
    }

    fn get_state(&self) -> flow_node::State {
        flow_node::State::ActivityState(State(
            self.flow_nodes.iter().map(FlowNode::get_state).collect(),
            self.state.clone(),
        ))
    }

    fn set_process(&mut self, process: process::Handle) {
        self.engine = Arc::new(process.model().script_engine_factory().create());
        self.log_broadcast.replace(process.log_broadcast());
        for flow_node in self.flow_nodes.iter_mut() {
            flow_node.set_process(process.clone());
        }
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

impl<T, E> ActivityContainer<T, E>
where
    T: Activity,
    E: ActivityType + Clone + Unpin,
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
                if self.variant != Variant::Executing {
                    self.flow_nodes[0].execute();
                    self.variant = Variant::Executing;
                }
                self.wake_when_ready(cx.waker().clone());
                self.get_mut().flow_nodes[0].poll_next_unpin(cx)
            }
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
                        me.flow_nodes[0].execute();
                        me.variant = Variant::Executing;
                    }
                    let result = me.flow_nodes[0].poll_next_unpin(cx);
                    match result {
                        Poll::Pending => {
                            me.wake_when_ready(cx.waker().clone());
                            Poll::Pending
                        }
                        Poll::Ready(action) => {
                            me.state.counter += 1;
                            if !matches!(test_before, Some(true)) {
                                me.variant = Variant::Testing;
                                me.spawn_test(expression);
                            }
                            Poll::Ready(action)
                        }
                    }
                } else {
                    me.variant = Variant::Testing;
                    me.wake_when_ready(cx.waker().clone());
                    me.spawn_test(expression);
                    Poll::Pending
                }
            }
            // TODO: implement `MultiInstanceLoopCharacteristics`
            _ => {
                self.wake_when_ready(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl<T, E> Stream for ActivityContainer<T, E>
where
    T: Activity,
    E: ActivityType + Unpin + Clone,
{
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.variant {
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
}
