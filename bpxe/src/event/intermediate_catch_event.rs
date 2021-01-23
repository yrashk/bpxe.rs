//! # Intermediate Catch Event flow node
use crate::bpmn::schema::{EventDefinitionType, FlowNodeType, IntermediateCatchEvent as Element};
use crate::event::ProcessEvent;
use crate::flow_node::{self, Action, FlowNode, IncomingIndex};
use crate::process;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::{broadcast, mpsc};
use tokio::task;

/// Intermediate Catch Event flow node
pub struct IntermediateCatchEvent {
    element: Arc<Element>,
    state: State,
    event_receivers: Vec<broadcast::Receiver<ProcessEvent>>,
    process: Option<process::Handle>,
    waker_sender: mpsc::Sender<Waker>,
    waker_receiver: Option<mpsc::Receiver<Waker>>,
}

impl IntermediateCatchEvent {
    /// Creates new Intermediate Catch Event flow node
    pub fn new(element: Element) -> Self {
        let (waker_sender, waker_receiver) = mpsc::channel(1);
        Self {
            element: Arc::new(element),
            state: State::Initialized,
            event_receivers: vec![],
            process: None,
            waker_sender,
            waker_receiver: Some(waker_receiver),
        }
    }

    /// Wakes IntermediateCatchEvent if there's an event available
    fn wake_on_event(&self, waker: Waker) {
        let waker_sender = self.waker_sender.clone();
        // This is done in a task to ensure completion, instead of
        // hoping try_send will complete.
        task::spawn(async move {
            let _ = waker_sender.send(waker).await;
        });
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Initialized,
    Ready,
    Complete,
    Done,
}

impl FlowNode for IntermediateCatchEvent {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::IntermediateCatchEvent(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&self) -> flow_node::State {
        flow_node::State::IntermediateCatchEvent(self.state.clone())
    }

    fn set_process(&mut self, process: process::Handle) {
        if let Some(mut waker_receiver) = self.waker_receiver.take() {
            let mut event_receiver = process.event_receiver();
            task::spawn(async move {
                let mut waker = None;
                let mut need_to_wake = false;
                loop {
                    tokio::select! {
                        mut waker_ = waker_receiver.recv() => if need_to_wake
                        {
                            if let Some(waker) = waker_.take() {
                                need_to_wake = false;
                                waker.wake();
                            }
                        } else {
                            waker = waker_;
                        },
                        _event = event_receiver.recv() => {
                            if let Some(waker) = waker.take() {
                                waker.wake();
                            } else {
                                need_to_wake = true;
                            }
                        }
                    }
                }
            });
        }

        self.process.replace(process);
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn incoming(&mut self, _index: IncomingIndex) {
        if let State::Initialized = self.state {
            self.state = State::Ready;
            if self.event_receivers.is_empty() && self.process.is_some() {
                // safe to unwrap here because we checked it above with `is_some`
                self.event_receivers = vec![self.process.as_ref().unwrap().event_receiver()];
            };
        }
    }
}

impl From<Element> for IntermediateCatchEvent {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for IntermediateCatchEvent {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state {
            State::Initialized => {
                self.wake_on_event(cx.waker().clone());
                Poll::Pending
            }
            State::Ready => {
                enum Control {
                    Ready(Vec<ProcessEvent>),
                    Done,
                }
                // FIXME: cloning this is not perfect, but it was a quick way
                // to get around mutable + immutable borrowing error
                let event_definitions = self.element.event_definitions.clone();
                let no_events = event_definitions.is_empty();
                let next = self.event_receivers.iter_mut().find_map(|receiver| {
                    let res = receiver.try_recv();
                    #[allow(unreachable_patterns)]
                    match res {
                        Ok(e @ ProcessEvent::NoneEvent) => {
                            if no_events {
                                Some(Control::Ready(vec![e]))
                            } else {
                                None
                            }
                        }
                        Ok(e) => {
                            // attempt matching events
                            let events: Vec<ProcessEvent> = event_definitions
                                .iter()
                                .filter_map(|event_definition| {
                                    use intertrait::cast::CastBox;
                                    if let Ok(definition) = event_definition
                                        .clone()
                                        .into_inner()
                                        .cast::<dyn EventDefinitionType>()
                                    {
                                        use std::convert::TryFrom;
                                        if let Ok(event) = ProcessEvent::try_from(definition) {
                                            if event == e {
                                                Some(e.clone())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if events.is_empty() {
                                None
                            } else {
                                Some(Control::Ready(events))
                            }
                        }
                        // If the channel is empty, continue trying
                        Err(broadcast::error::TryRecvError::Empty) => None,
                        // If event broadcaster is closed, we'll never receive anything
                        Err(broadcast::error::TryRecvError::Closed) => Some(Control::Done),
                        // If event receiver has lagged, continue trying
                        Err(broadcast::error::TryRecvError::Lagged(_)) => None,
                    }
                });
                match next {
                    Some(Control::Ready(_)) => {
                        self.state = State::Complete;
                        Poll::Ready(Some(Action::Flow(
                            (0..self.element.outgoings().len()).collect(),
                        )))
                    }
                    Some(Control::Done) => Poll::Ready(None),
                    None => {
                        self.wake_on_event(cx.waker().clone());
                        Poll::Pending
                    }
                }
            }
            State::Complete => {
                self.state = State::Done;
                Poll::Ready(Some(Action::Complete))
            }
            State::Done => {
                self.wake_on_event(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bpmn::parse;
    use crate::event::ProcessEvent;
    use crate::model;
    use crate::process::Log;
    use crate::test::*;

    #[tokio::test]
    async fn catch_none_event() {
        let definitions = parse(include_str!("test_models/catch_none_event.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        assert!(log_mailbox
            .receive(|e| matches!(e, Log::FlowNodeIncoming { node, .. } if node.id().as_ref().unwrap() == "catch"))
            .await);

        let _ = handle.event_broadcast().send(ProcessEvent::NoneEvent);

        assert!(
            mailbox
                .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "signal"))
                .await
        );
    }

    #[tokio::test]
    async fn catch_signal_event() {
        let definitions = parse(include_str!("test_models/catch_signal_event.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        assert!(log_mailbox
            .receive(|e| matches!(e, Log::FlowNodeIncoming { node, .. } if node.id().as_ref().unwrap() == "catch"))
            .await);

        let _ = handle.event_broadcast().send(ProcessEvent::SignalEvent {
            signal_ref: Some("signal".into()),
        });

        assert!(
            mailbox
                .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "report"))
                .await
        );
    }
}
