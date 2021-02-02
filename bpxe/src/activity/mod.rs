//! Activity
use crate::bpmn::schema::{
    self, ActivityType, BaseElementType, DataAssociationType, DataInput, DataOutput,
    DocumentElementContainer, Expr, FlowNodeType, FormalExpression, InputOutputSpecification,
    InputOutputSpecificationType, InputSetType, LoopCharacteristics,
    MultiInstanceLoopCharacteristics, MultiInstanceLoopCharacteristicsInputDataItem,
    MultiInstanceLoopCharacteristicsLoopCardinality,
    MultiInstanceLoopCharacteristicsOutputDataItem, OutputSetType, SequenceFlow,
    StandardLoopCharacteristics, StandardLoopCharacteristicsLoopCondition,
};
use crate::data_object::{self, DataObject, DataObjectExt};
use crate::flow_node::{self, Action, FlowNode, IncomingIndex, OutgoingIndex, StateError};
use crate::language::{Engine as _, EngineContextProvider, MultiLanguageEngine};
use crate::process::{self, Log};
use factory::ParameterizedFactory;
use futures::stream::{Stream, StreamExt};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    /// Reports input sets
    ///
    /// Default implementation does nothing
    #[allow(unused_variables)]
    fn input_sets(&mut self, input_sets: Vec<InputSet>) {}

    /// Polls for output sets
    ///
    /// [`ActivityContainer`] expects this function to be non-idempotent, that is, the
    /// implementation should not return the same output set twice. The output sets stored
    /// in the implementation should be moved out via `Some(...)`.
    ///
    /// Default implementation returns `None`
    fn take_output_sets(&mut self) -> Option<Vec<OutputSet>> {
        None
    }
}

type NamedDataObjects = HashMap<String, Box<dyn DataObject>>;
type InputDataItem = Option<(Option<String>, Box<dyn DataObject>)>;

/// Optionally named input set
pub type InputSet = (Option<String>, Vec<Box<dyn DataObject>>);

/// Optionally named output set
pub type OutputSet = (Option<String>, Vec<Box<dyn DataObject>>);

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
    properties: HashMap<String, Box<dyn DataObject>>,
    // Per-instance input data items:
    // (token, input_data_item) => data_object
    input_data_items: HashMap<(usize, Option<String>), Box<dyn DataObject>>,
}

enum Variant<T> {
    // preparing instances
    Initializing {
        // returns activity with an optional (named) input_data_item
        activities: oneshot::Receiver<(IncomingIndex, Vec<(InputDataItem, T)>)>,
    },
    // waiting for data to be retrieved
    AwaitingData {
        incoming: IncomingIndex,
        data: oneshot::Receiver<NamedDataObjects>,
    },
    // waiting for an incoming flow
    Initialized,
    // incoming flow has been registered
    Ready {
        // for standard loop
        test_before: bool,
    },
    // activity is being executed
    Executing,
    // awaiting output completion
    AwaitingOutputs {
        // action to send once outputs are completed
        action: Option<Poll<Option<Action>>>,
        // this signals output propagation completion
        receiver: oneshot::Receiver<()>,
        // variant to change to
        next_variant: Box<Option<Variant<T>>>,
    },
    // loopCondition is being tested (standard loop)
    Testing,
    // error happened
    Errored,
    // done
    Done,
}

#[derive(Clone, Debug)]
enum Completion {
    Success(bool),
    Error,
}

impl<T, E, F> ActivityContainer<T, E, F>
where
    T: Activity + 'static,
    E: ActivityType + Clone + Unpin,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin + 'static,
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
            flow_node_tokens.push(flow_nodes.insert(activity));
        }

        let properties = element
            .properties()
            .iter()
            .filter(|property| property.id().is_some())
            .map(|property| {
                (
                    property.id().as_ref().unwrap().clone(),
                    Box::new(data_object::Empty) as Box<dyn DataObject>,
                )
            })
            .collect();

        let input_data_items = HashMap::new();

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
            properties,
            input_data_items,
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

    fn prepare_instances(&mut self, index: IncomingIndex) {
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
                            loop_cardinality:
                                Some(MultiInstanceLoopCharacteristicsLoopCardinality(
                                    Expr::FormalExpression(expr),
                                )),
                            ..
                        },
                    )) => {
                        let engine = process.model().expression_engine_factory().create();
                        match engine.eval::<usize>(expr, &mut engine.new_context()).await {
                            Ok(cardinality) => (0..cardinality)
                                .into_iter()
                                .map(|_| (None, flow_node_maker.create(element.clone())))
                                .collect(),
                            Err(err) => {
                                let _ = process.log_broadcast().send(Log::ExpressionError {
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
                            input_data_item,
                            ..
                        },
                    )) => {
                        let input_data_item = match input_data_item {
                            Some(MultiInstanceLoopCharacteristicsInputDataItem(DataInput {
                                id: Some(input_data_item),
                                ..
                            })) => Some(input_data_item),
                            _ => None,
                        };

                        match process.data_object(loop_data_input).await {
                            Ok(data_object) => {
                                let data_object = data_object.read().await;
                                if let Some(collection) =
                                    data_object.downcast_ref::<data_object::Collection>()
                                {
                                    collection
                                        .iter()
                                        .map(|data_object| {
                                            (
                                                Some((
                                                    input_data_item.cloned(),
                                                    dyn_clone::clone_box(&**data_object),
                                                )),
                                                flow_node_maker.create(element.clone()),
                                            )
                                        })
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
        }
    }

    fn request_data(&mut self, incoming: IncomingIndex) {
        if let Some(process) = self.process.clone() {
            let (sender, receiver) = oneshot::channel();
            let element = self.element.clone();
            let waker_receiver = self.waker_receiver.clone();
            task::spawn(async move {
                // Process data input association
                let input_associations = element.data_input_associations();
                let mut data = HashMap::new();
                for association in input_associations {
                    let sources = association.source_refs();
                    for source in sources {
                        // This is where the data object will be
                        let mut data_object = None;
                        // Check data objects
                        if let Ok(data_object_) = process.data_object(source).await {
                            let read = data_object_.read().await;
                            data_object = Some(dyn_clone::clone_box(&**read));
                        }
                        if let Some(data_object) = data_object {
                            data.insert(association.target_ref.to_owned(), data_object);
                        }
                    }
                }
                let waker = waker_receiver.borrow();
                if let Some(waker) = waker.as_ref() {
                    waker.wake_by_ref();
                }
                let _ = sender.send(data);
            });
            self.variant = Variant::AwaitingData {
                data: receiver,
                incoming,
            };
        }
    }

    fn connect_inputs(&mut self, data: &NamedDataObjects) {
        let input_associations = self.element.data_input_associations();
        for token in self.flow_node_tokens.iter() {
            if let Some(flow_node) = self.flow_nodes.get_mut(*token) {
                let io_spec = self.element.io_specification();
                let mut data_inputs = match io_spec {
                    None => HashMap::new(),
                    Some(InputOutputSpecification { data_inputs, .. }) => data_inputs
                        .iter()
                        .filter_map(|data_input| data_input.id().as_ref())
                        .map(|id| (id, Box::new(data_object::Empty) as Box<dyn DataObject>))
                        .collect(),
                };

                for association in input_associations {
                    let sources = association.source_refs();
                    if !sources.is_empty() {
                        // TODO: handle transformations (and more than one source)
                        // (for now we just assume there could be only one)
                        let source = &sources[0];
                        // figure out the source
                        let mut item = None;
                        // Check input data items
                        if let Some(item_) = self
                            .input_data_items
                            .get(&(*token, Some(source.to_string())))
                        {
                            item.replace(dyn_clone::clone_box(&*item_));
                        }
                        // Check properties
                        if let Some(item_) = self.properties.get(source) {
                            item.replace(dyn_clone::clone_box(&*item_));
                        }
                        // Check data objects
                        if let Some(item_) = data.get(association.target_ref()) {
                            item.replace(dyn_clone::clone_box(&*item_));
                        }
                        // TODO: process properties

                        if let Some(item) = item {
                            // figure out the destination
                            // is it a data input?
                            if let Some(data_object) = data_inputs.get_mut(association.target_ref())
                            {
                                *data_object = *item;
                            } else if let Some(data_object) =
                                self.properties.get_mut(association.target_ref())
                            {
                                // is it a property?
                                *data_object = *item;
                            }
                        }
                    }
                }

                // Collect input sets
                let input_sets = match io_spec {
                    None => vec![],
                    Some(InputOutputSpecification { input_sets, .. }) => input_sets
                        .iter()
                        .map(|input_set| {
                            (
                                input_set.id().clone(),
                                input_set
                                    .data_input_refses()
                                    .iter()
                                    .filter_map(|name| {
                                        if let Some(data_input) = data_inputs.remove(name) {
                                            return Some(data_input);
                                        }
                                        None
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                };
                flow_node.input_sets(input_sets);
            }
        }
    }

    fn connect_outputs(
        &mut self,
        cx: &mut Context<'_>,
        token: usize,
        action: Poll<Option<Action>>,
        next_variant: Option<Variant<T>>,
        remove_token: bool,
    ) -> Poll<Option<Action>> {
        if let Some(process) = self.process.clone() {
            let process_definition = process.element();
            if let Some(flow_node) = self.flow_nodes.get_mut(token) {
                let mut async_associations = vec![];
                if let Some(output_sets) = flow_node.take_output_sets() {
                    let mut output_data_item: Box<dyn DataObject> = Box::new(data_object::Empty);
                    let output_associations = self.element.data_output_associations().clone();
                    // Process what we can in sync as we have access to &mut self
                    for association in output_associations {
                        let sources = association.source_refs();
                        if !sources.is_empty() {
                            // TODO: handle transformations (and more than one source)
                            // (for now we just assume there could be only one)
                            let source = &sources[0];
                            // figure out the source
                            let mut item = None;
                            // is it data output?
                            for (id, output_set) in &output_sets {
                                // find matching output set definition
                                if self.element.io_specification().is_some() {
                                    if let Some(set) = self
                                        .element
                                        .io_specification()
                                        .as_ref()
                                        // it's ok to unwrap, we checked above
                                        .unwrap()
                                        .output_sets()
                                        .iter()
                                        .find(|set| &set.id == id)
                                    {
                                        // scan through data output refs
                                        if let Some((_, data_object)) = set
                                            .data_output_refses()
                                            .iter()
                                            .zip(output_set.iter())
                                            .find(|(name, _)| name == &source)
                                        {
                                            item.replace(dyn_clone::clone_box(&*data_object));
                                            break;
                                        }
                                    }
                                }
                            }
                            // is it a property?
                            if let Some(item_) = self.properties.get(source) {
                                item.replace(dyn_clone::clone_box(&*item_));
                            }
                            if let Some(LoopCharacteristics::MultiInstanceLoopCharacteristics(
                                MultiInstanceLoopCharacteristics {
                                    output_data_item:
                                        Some(MultiInstanceLoopCharacteristicsOutputDataItem(
                                            DataOutput { id: Some(id), .. },
                                        )),
                                    ..
                                },
                            )) = self.element.loop_characteristics()
                            {
                                // is it an output data item?
                                if id == source {
                                    item.replace(dyn_clone::clone_box(&output_data_item));
                                }
                            }
                            if let Some(item) = item {
                                // figure out the destination
                                if let Some(data_object) =
                                    self.properties.get_mut(association.target_ref())
                                {
                                    // is it a property?
                                    *data_object = *item;
                                    continue;
                                } else if let Some(
                                    LoopCharacteristics::MultiInstanceLoopCharacteristics(
                                        MultiInstanceLoopCharacteristics {
                                            output_data_item:
                                                Some(MultiInstanceLoopCharacteristicsOutputDataItem(
                                                    DataOutput { id: Some(id), .. },
                                                )),
                                            ..
                                        },
                                    ),
                                ) = self.element.loop_characteristics()
                                {
                                    // is it an output data item?
                                    if id == association.target_ref() {
                                        output_data_item = *item;
                                        continue;
                                    }
                                }

                                // is it a data object?
                                if let Some(true) = process_definition
                                    .find_by_id(association.target_ref())
                                    .map(|e| {
                                        e.downcast_ref::<schema::DataObject>().is_some()
                                            || e.downcast_ref::<schema::DataObjectReference>()
                                                .is_some()
                                    })
                                {
                                    async_associations
                                        .push((association.target_ref().to_string(), item));
                                }
                            }
                        }
                    }
                }

                // if there's anything that requires sync, spawn a task
                if !async_associations.is_empty() {
                    let (sender, receiver) = oneshot::channel();
                    let is_variant_none = next_variant.is_none();
                    let old_variant = std::mem::replace(
                        &mut self.variant,
                        Variant::AwaitingOutputs {
                            action: Some(action),
                            receiver,
                            next_variant: Box::new(next_variant),
                        },
                    );

                    // Save the old variant
                    if is_variant_none {
                        if let Variant::AwaitingOutputs {
                            ref mut next_variant,
                            ..
                        } = self.variant
                        {
                            **next_variant = Some(old_variant);
                        }
                    }
                    let waker_receiver = self.waker_receiver.clone();
                    task::spawn(async move {
                        {
                            while let Some((data_object_name, new_object)) =
                                async_associations.pop()
                            {
                                if let Ok(data_object) =
                                    process.data_object(&data_object_name).await
                                {
                                    let mut write = data_object.write().await;
                                    write.send(*new_object);
                                }
                            }
                            let waker = waker_receiver.borrow();
                            if let Some(waker) = waker.as_ref() {
                                waker.wake_by_ref();
                            }
                            let _ = sender.send(());
                        }
                    });
                    // Bail so that we don't send the next_variant
                    self.wake_when_ready(cx.waker().clone());
                    if remove_token {
                        self.remove_token(token, true);
                    }
                    return Poll::Pending;
                }
            }
            if remove_token {
                self.remove_token(token, true);
            }
        }
        // If we were not able to get a flow node or there were no output sets,
        // or there was no need to use async for propagation,
        // simply change to next_variant
        if let Some(variant) = next_variant {
            self.variant = variant;
        }
        // ...and simply return the given action
        if action.is_pending() {
            self.wake_when_ready(cx.waker().clone());
        }
        action
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
            if self.flow_nodes.is_empty() {
                // If we need to launch a multi-instance activity, we need to populate the flow nodes
                // upon activation
                self.prepare_instances(index);
                return;
            } else {
                // Otherwise, we're ready to request data
                self.request_data(index);
                self.wake();
                return;
            }
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
    T: Activity + 'static,
    E: ActivityType + Clone + Unpin,
    F: ParameterizedFactory<Item = T, Parameter = E> + Send + Clone + Unpin + 'static,
{
    fn spawn_test(&self, expression: FormalExpression) {
        let engine = self.engine.clone();
        let notifier = self.notifier.clone();
        let log_broadcast = self.log_broadcast.clone();
        let waker_receiver = self.waker_receiver.clone();
        task::spawn(async move {
            let result = engine.eval(&expression, &mut engine.new_context()).await;
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
                match self.flow_nodes.poll_next_unpin(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Ready(Some((StreamYield::Item(item), token))) => {
                        if let Action::Complete = item {
                            self.variant = Variant::Done;
                            Poll::Pending
                        } else {
                            self.connect_outputs(cx, token, Poll::Ready(Some(item)), None, false)
                        }
                    }
                    Poll::Ready(Some((StreamYield::Finished(stream), _))) => {
                        stream.remove(Pin::new(&mut self.flow_nodes));
                        Poll::Ready(None)
                    }
                }
            }
            // Standard loop
            //   - with loopCondition specified
            Some(LoopCharacteristics::StandardLoopCharacteristics(
                StandardLoopCharacteristics {
                    test_before,
                    loop_maximum,
                    loop_condition:
                        Some(StandardLoopCharacteristicsLoopCondition(Expr::FormalExpression(
                            expression,
                        ))),
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
                            (StreamYield::Item(action), token) => {
                                let should_test = !matches!(test_before, Some(true));
                                me.state.counter += 1;
                                if should_test {
                                    me.spawn_test(expression);
                                }
                                me.connect_outputs(
                                    cx,
                                    token,
                                    Poll::Ready(Some(action)),
                                    if should_test {
                                        Some(Variant::Testing)
                                    } else {
                                        None
                                    },
                                    false,
                                )
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
                            me.connect_outputs(
                                cx,
                                token,
                                Poll::Ready(Some(action)),
                                if me.flow_nodes.len() == 1 {
                                    Some(Variant::Done)
                                } else {
                                    Some(Variant::Ready { test_before: false })
                                },
                                true,
                            )
                        }
                        (StreamYield::Finished(finished), token) => {
                            finished.remove(Pin::new(&mut me.flow_nodes));
                            me.variant = Variant::Initialized;
                            me.remove_token(token, false);
                            Poll::Ready(Some(Action::Complete))
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
                            me.connect_outputs(
                                cx,
                                token,
                                Poll::Ready(Some(action)),
                                if me.flow_nodes.len() == 1 {
                                    Some(Variant::Done)
                                } else {
                                    None
                                },
                                true,
                            )
                        }
                        (StreamYield::Finished(finished), token) => {
                            finished.remove(Pin::new(&mut me.flow_nodes));
                            me.variant = Variant::Initialized;
                            me.remove_token(token, false);
                            Poll::Ready(Some(Action::Complete))
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
                    for (data_object, mut activity) in activities.drain(..) {
                        // Set up activity:
                        // 1. Process
                        if let Some(process) = me.process.clone() {
                            activity.set_process(process);
                        }

                        let token = me.flow_nodes.insert(activity);
                        me.flow_node_tokens.push(token);

                        // 2. Save input data
                        if let Some((id, data_object)) = data_object {
                            me.input_data_items.insert((token, id), data_object);
                        }

                        // 3. TODO: figure out deterministic state recovery
                        // activity.set_state(...);
                    }
                    me.wake_when_ready(cx.waker().clone());
                    me.request_data(index);
                    Poll::Pending
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
            Variant::AwaitingData {
                ref mut data,
                incoming,
            } => {
                match data.try_recv() {
                    Ok(ref data) => {
                        self.connect_inputs(data);
                        self.variant = Variant::Ready { test_before: false };
                        self.incoming(incoming);
                        self.poll_next(cx)
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
                }
            }
            Variant::AwaitingOutputs {
                ref mut receiver,
                ref mut next_variant,
                ref mut action,
            } => {
                match receiver.try_recv() {
                    Ok(()) => {
                        // it's ok to unwrap here because connect_outputs ensures there isn't going
                        // to be a None
                        let result = action.take().unwrap();
                        // ...as well as here
                        self.variant = next_variant.take().unwrap();
                        if result.is_pending() {
                            self.wake_when_ready(cx.waker().clone());
                        }
                        result
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
                }
            }
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
                Poll::Ready(Some(Action::Complete))
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::bpmn::{parse, schema::*};
    use crate::data_object;
    use crate::language::*;
    use crate::model;
    use crate::process::Log;
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

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn multi_loop_data_object_input() {
        let definitions = parse(include_str!(
            "test_models/multi_loop_data_object_input.bpmn"
        ))
        .unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn(
                    "notify",
                    move |i: u8| while let Err(_) = sender.try_send(i) {},
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
                Box::new(data_object::Container(1u8)),
                Box::new(data_object::Container(2u8)),
                Box::new(data_object::Container(3u8)),
            ]));
        }

        assert!(handle.start().await.is_ok());

        // Should receive the data inputs
        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(1));
        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(2));
        assert_eq!(timeout(receiver.recv()).await.unwrap(), Some(3));

        // It should stop when cardinality is exhausted
        assert!(timeout(receiver.recv()).await.is_err());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn multi_loop_data_object_output() {
        let definitions = parse(include_str!(
            "test_models/multi_loop_data_object_output.bpmn"
        ))
        .unwrap();
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                rhai_engine.register_fn("notify", move |_: u8| {});
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        {
            let data_object = handle.data_object("data_object").await.unwrap();

            let mut write = data_object.write().await;
            *write = Box::new(data_object::Collection(vec![
                Box::new(data_object::Container(1u8)),
                Box::new(data_object::Container(2u8)),
                Box::new(data_object::Container(3u8)),
            ]));
        }

        assert!(handle.start().await.is_ok());

        assert!(timeout(
            log_mailbox.receive(|m| matches!(m, Log::FlowNodeCompleted { node, ..}
                if matches!(node.downcast_ref::<ScriptTask>(), Some(ScriptTask { id, .. }) if id.as_ref().unwrap() == "script")))
        )
        .await
        .is_ok());

        let collection =
            match dyn_clone::clone_box(&*handle.data_object("output").await.unwrap().read().await)
                .downcast::<data_object::Collection>()
            {
                Ok(coll) => coll,
                _ => panic!("should be a collection"),
            };

        assert_eq!(collection.len(), 3);
        assert!(matches!(
            collection[0].downcast_ref::<data_object::Container<i64>>(),
            Some(data_object::Container(1))
        ));
        assert!(matches!(
            collection[1].downcast_ref::<data_object::Container<i64>>(),
            Some(data_object::Container(2))
        ));
        assert!(matches!(
            collection[2].downcast_ref::<data_object::Container<i64>>(),
            Some(data_object::Container(3))
        ));
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn input_data_object() {
        let definitions = parse(include_str!("test_models/activity_io.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn("notify", move |i: ::rhai::Array| {
                    while let Err(_) = sender.try_send(i[0].clone()) {}
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        {
            let data_object = handle.data_object("data_object").await.unwrap();

            let mut write = data_object.write().await;
            *write = Box::new(data_object::Container(1u8));
        }

        assert!(handle.start().await.is_ok());

        // Should receive the data inputs
        assert_eq!(
            timeout(receiver.recv())
                .await
                .unwrap()
                .unwrap()
                .cast::<u8>(),
            1
        );
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn input_property() {
        let definitions = parse(include_str!("test_models/activity_io.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                let sender = sender.clone();
                rhai_engine.register_fn("notify", move |i: ::rhai::Array| {
                    while let Err(_) = sender.try_send(i[1].clone()) {}
                });
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        {
            let data_object = handle.data_object("data_object").await.unwrap();

            let mut write = data_object.write().await;
            *write = Box::new(data_object::Container(1u8));
        }

        assert!(handle.start().await.is_ok());

        // Should receive the data inputs
        assert_eq!(
            timeout(receiver.recv())
                .await
                .unwrap()
                .unwrap()
                .cast::<u8>(),
            1
        );
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn output_data_object() {
        let definitions = parse(include_str!("test_models/activity_io.bpmn")).unwrap();
        let model = model::Model::new(definitions).with_script_engine_factory(
            model::FnLanguageEngineFactory(move || {
                use ::rhai::RegisterFn;
                let mut engine = MultiLanguageEngine::new();
                let rhai_engine = engine.rhai.engine_mut().unwrap();
                rhai_engine.register_fn("notify", move |_: ::rhai::Array| {});
                engine
            }),
        );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        {
            let data_object = handle.data_object("data_object").await.unwrap();

            let mut write = data_object.write().await;
            *write = Box::new(data_object::Container(1u8));
        }

        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        assert!(timeout(
            log_mailbox.receive(|m| matches!(m, Log::FlowNodeCompleted { node, ..}
                if matches!(node.downcast_ref::<ScriptTask>(), Some(ScriptTask { id, .. }) if id.as_ref().unwrap() == "activity")))
        )
        .await
        .is_ok());

        assert!(matches!(
            handle
                .data_object("output")
                .await
                .unwrap()
                .read()
                .await
                .downcast_ref::<data_object::Container<i64>>(),
            Some(data_object::Container(1))
        ));
    }
}
