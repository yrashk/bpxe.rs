//! # Script Task flow node
use crate::activity::{Activity, InputSet, OutputSet};
use crate::bpmn::schema::{FlowNodeType, ScriptTask as Element};

use crate::data_object::{self, DataObject};
use crate::flow_node::{self, Action, FlowNode};
use crate::language::{
    Engine as _, EngineContext, EngineContextProvider, EvaluationError, MultiLanguageEngine,
};
use crate::process::Log;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::broadcast;
use tokio::task;

/// Script Task flow node
pub struct Task {
    element: Arc<Element>,
    state: State,
    engine: Arc<MultiLanguageEngine>,
    waker: Option<Waker>,
    notifier: broadcast::Sender<Completion>,
    notifier_receiver: broadcast::Receiver<Completion>,
    log_broadcast: Option<broadcast::Sender<Log>>,
    input_sets: Vec<InputSet>,
    output_sets: Option<Vec<OutputSet>>,
}

#[derive(Clone)]
enum Completion {
    Success(Option<Vec<OutputSet>>),
    Error,
}

impl Task {
    /// Creates new Script Task flow node
    pub fn new(element: Element) -> Self {
        let (notifier, notifier_receiver) = broadcast::channel(1);
        Self {
            element: Arc::new(element),
            state: State::Initialized,
            engine: Arc::new(Default::default()),
            waker: None,
            notifier,
            notifier_receiver,
            log_broadcast: None,
            input_sets: vec![],
            output_sets: None,
        }
    }

    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Initialized,
    Ready,
    Execute,
    Executing,
    Errored,
    Done,
}

impl FlowNode for Task {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::ScriptTask(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&mut self) -> flow_node::State {
        flow_node::State::ScriptTask(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn set_process(&mut self, process: crate::process::Handle) {
        if let State::Initialized = self.state {
            self.state = State::Ready;
            self.log_broadcast.replace(process.log_broadcast());
            self.engine = Arc::new(process.model().script_engine_factory().create());
            self.wake();
        }
    }
}

impl Activity for Task {
    fn execute(&mut self) {
        self.state = State::Execute;
        self.wake();
    }

    fn input_sets(&mut self, input_sets: Vec<InputSet>) {
        self.input_sets = input_sets;
    }

    fn take_output_sets(&mut self) -> Option<Vec<OutputSet>> {
        self.output_sets.take()
    }
}

impl From<Element> for Task {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for Task {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state {
            State::Initialized => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Ready => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Execute => {
                self.state = State::Executing;
                let waker = cx.waker().clone();
                let engine = self.engine.clone();
                let element = self.element.as_ref().clone();
                let notifier = self.notifier.clone();
                let log_broadcast = self.log_broadcast.clone();
                let mut context = engine.new_context();
                // We only need input once, we can drain it
                for (name, input_set) in
                    std::mem::replace(&mut self.input_sets, Vec::new()).drain(..)
                {
                    if let Some(name) = name {
                        context.set(&name, Box::new(data_object::Collection(input_set)));
                    } else {
                        context.set("input", Box::new(data_object::Collection(input_set)));
                    }
                }

                task::spawn(async move {
                    match engine
                        .eval::<Vec<Box<dyn DataObject>>>(&element, &mut context)
                        .await
                    {
                        Ok(data_objects) => {
                            let _ = notifier
                                .send(Completion::Success(Some(vec![(None, data_objects)])));
                        }
                        Err(EvaluationError::ResultTypeError { got, .. }) if got == "()" => {
                            let _ = notifier.send(Completion::Success(None));
                        }
                        Err(err) => {
                            let _ = notifier.send(Completion::Error);
                            if let Some(log_broadcast) = log_broadcast {
                                let _ = log_broadcast.send(Log::ScriptError {
                                    error: format!("{:?}", err),
                                });
                            }
                        }
                    }
                    waker.wake();
                });
                Poll::Pending
            }
            State::Executing => match self.notifier_receiver.try_recv() {
                Ok(Completion::Success(data_objects)) => {
                    self.output_sets = data_objects;
                    self.waker.replace(cx.waker().clone());
                    self.state = State::Done;
                    Poll::Ready(Some(Action::Flow(
                        (0..self.element.outgoings().len()).collect(),
                    )))
                }
                Ok(Completion::Error) => {
                    self.state = State::Errored;
                    Poll::Ready(Some(Action::Complete))
                }
                Err(broadcast::error::TryRecvError::Empty) => {
                    self.waker.replace(cx.waker().clone());
                    Poll::Pending
                }
                Err(broadcast::error::TryRecvError::Lagged(_)) => {
                    self.waker.replace(cx.waker().clone());
                    Poll::Pending
                }
                Err(broadcast::error::TryRecvError::Closed) => Poll::Ready(None),
            },
            State::Errored => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Done => {
                self.state = State::Ready;
                Poll::Ready(Some(Action::Complete))
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
    use tokio::sync::mpsc;

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn runs() {
        let definitions = parse(include_str!("test_models/task_script.bpmn")).unwrap();
        let (sender, mut receiver) = mpsc::channel(10);
        let sender_clone = sender.clone();
        let model =
            model::Model::new(definitions).with_script_engine_factory(
                model::FnLanguageEngineFactory(move || {
                    use ::rhai::RegisterFn;
                    let mut engine = MultiLanguageEngine::new();
                    let rhai_engine = engine.rhai.engine_mut().unwrap();
                    let sender_clone = sender_clone.clone();
                    rhai_engine.register_fn("notify", move || {
                        while let Err(_) = sender_clone.try_send(()) {}
                    });
                    engine
                }),
            );

        let model = model.spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        assert!(handle.start().await.is_ok());

        assert_eq!(receiver.recv().await, Some(()));
    }
}
