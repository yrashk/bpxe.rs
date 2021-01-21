//! # Start Event flow node
use crate::bpmn::schema::{FlowNodeType, StartEvent as Element};
use crate::event::ProcessEvent;
use crate::flow_node::{self, Action, FlowNode};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::{broadcast, mpsc};
use tokio::task;

/// Start Event flow node
pub struct StartEvent {
    element: Arc<Element>,
    state: State,
    event_receivers: Vec<broadcast::Receiver<ProcessEvent>>,
    waker_sender: mpsc::Sender<Waker>,
    waker_receiver: Option<mpsc::Receiver<Waker>>,
}

impl StartEvent {
    /// Creates new Start Event flow node
    pub fn new(element: Element) -> Self {
        let (waker_sender, waker_receiver) = mpsc::channel(1);
        Self {
            element: Arc::new(element),
            state: State::Initialized,
            event_receivers: vec![],
            waker_sender,
            waker_receiver: Some(waker_receiver),
        }
    }

    /// Wakes StartEvent if there's an event available
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

impl FlowNode for StartEvent {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::StartEvent(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&self) -> flow_node::State {
        flow_node::State::StartEvent(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn set_process(&mut self, process: crate::process::Handle) {
        if let State::Initialized = self.state {
            self.state = State::Ready;
            if let Some(mut waker_receiver) = self.waker_receiver.take() {
                let mut event_receiver = process.event_receiver();
                task::spawn(async move {
                    let mut waker = None;
                    loop {
                        tokio::select! {
                            waker_ = waker_receiver.recv() => {
                                waker = waker_;
                            }
                            _event = event_receiver.recv() => {
                                // FIXME: should we only wake if it's a `Start` event
                                // or should we wake on any event as we do now?
                                if let Some(waker) = waker.take() {
                                    waker.wake();
                                }
                            }
                        }
                    }
                });
            }
            self.event_receivers.push(process.event_receiver());
        }
    }
}

impl From<Element> for StartEvent {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for StartEvent {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state {
            State::Initialized => Poll::Pending,
            State::Ready => {
                enum Control {
                    Ready,
                    Done,
                }
                let next = self.event_receivers.iter_mut().find_map(|receiver| {
                    let res = receiver.try_recv();
                    #[allow(unreachable_patterns)]
                    match res {
                        Ok(ProcessEvent::Start) => Some(Control::Ready),
                        // If event doesn't match, keep it pending
                        Ok(_) => None,
                        // If the channel is empty, continue trying
                        Err(broadcast::error::TryRecvError::Empty) => None,
                        // If event broadcaster is closed, we'll never receive anything
                        Err(broadcast::error::TryRecvError::Closed) => Some(Control::Done),
                        // If event receiver has lagged, continue trying
                        Err(broadcast::error::TryRecvError::Lagged(_)) => None,
                    }
                });
                match next {
                    Some(Control::Ready) => {
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
    use crate::bpmn::schema::*;
    use crate::model;
    use crate::process::Log;
    use crate::test::Mailbox;

    #[tokio::test]
    async fn start_flows() {
        let mut definitions = Definitions {
            root_elements: vec![Process {
                id: Some("proc1".into()),
                flow_elements: vec![
                    StartEvent {
                        id: Some("start".into()),
                        ..Default::default()
                    }
                    .into(),
                    EndEvent {
                        id: Some("end".into()),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into()],
            ..Default::default()
        };

        definitions
            .find_by_id_mut("proc1")
            .unwrap()
            .downcast_mut::<Process>()
            .unwrap()
            .establish_sequence_flow("start", "end", "s1", None)
            .unwrap();

        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        assert!(
            mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<StartEvent>(),
                    Some(start_event) if start_event.id().as_ref().unwrap() == "start")
                } else {
                    false
                })
                .await
        );

        assert!(
            mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<EndEvent>(),
                    Some(end_event) if end_event.id().as_ref().unwrap() == "end")
                } else {
                    false
                })
                .await
        );
    }
}
