//! # Model
//!
//! Model is a central entrypoint to BPMN execution. It contains all definitions of a BPMN document
//! and orchestrates process instantiation and execution.
use crate::bpmn::schema::{Definitions, RootElement};
use crate::process;
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::{self, JoinHandle};

use thiserror::Error;

/// Model error
#[derive(Error, Debug)]
pub enum Error {
    /// Error while receiving response to a request
    #[error("error while receiving response to a request: {error:?}")]
    ResponseRecvError {
        #[from]
        error: oneshot::error::RecvError,
    },
}

/// Model is a container for a BPMN document
pub struct Model {
    definitions: Arc<Definitions>,
    processes: Vec<process::Handle>,
}

/// Control handle for a running model
#[derive(Clone, Debug)]
pub struct Handle {
    definitions: Arc<Definitions>,
    sender: mpsc::Sender<Request>,
    log_broadcast: broadcast::Sender<Log>,
}

/// Model events
#[derive(Clone, Debug)]
pub enum Log {}

enum Request {
    JoinHandle(JoinHandle<()>),
    Terminate(oneshot::Sender<Option<JoinHandle<()>>>),
    Processes(oneshot::Sender<Vec<process::Handle>>),
}

impl Model {
    /// Initializes a model
    ///
    /// In order to make it operational, use [`Model::spawn`]
    pub fn new(definitions: Definitions) -> Self {
        Self {
            definitions: Arc::new(definitions),
            processes: vec![],
        }
    }

    /// Spawns model operation task
    pub async fn spawn(self) -> Handle {
        let (sender, receiver) = mpsc::channel(1);
        let (log_broadcast, _) = broadcast::channel(128);
        let log_sender = log_broadcast.clone();
        let handle = Handle {
            definitions: self.definitions.clone(),
            sender: sender.clone(),
            log_broadcast,
        };

        let handle_clone = handle.clone();

        let join_handle =
            task::spawn(async move { self.runner(receiver, handle_clone, log_sender).await });
        let _ = sender.send(Request::JoinHandle(join_handle)).await;

        handle
    }

    // Main loop
    async fn runner(
        mut self,
        mut receiver: mpsc::Receiver<Request>,
        handle: Handle,
        _log_broadcast: broadcast::Sender<Log>,
    ) {
        // We save our own join handle as we want to return it
        // exclusively to the termination requester
        let mut join_handle = None;
        // Initialize
        self.processes = join_all(
            self.definitions
                .root_elements
                .iter()
                .filter_map(|e| match e {
                    RootElement::Process(def) => {
                        Some(process::Process::new(def.clone(), handle.clone()).spawn())
                    }
                    _ => None,
                }),
        )
        .await;

        // Process requests until termination
        loop {
            let next = receiver.recv().await;
            match next {
                Some(Request::JoinHandle(handle)) => join_handle = Some(handle),
                Some(Request::Terminate(sender)) => {
                    let _ = sender.send(join_handle.take());
                    return;
                }
                Some(Request::Processes(sender)) => {
                    let _ = sender.send(self.processes.clone());
                }
                None => {}
            }
        }
    }
}

impl Handle {
    /// Request and wait for model execution termination
    pub async fn terminate(self) {
        let (sender, receiver) = oneshot::channel();
        let _ = self.sender.send(Request::Terminate(sender)).await;
        if let Ok(Some(handle)) = receiver.await {
            let _ = handle.await;
        }
    }

    /// Returns model definitions
    pub fn definitions(&self) -> Arc<Definitions> {
        self.definitions.clone()
    }

    /// Returns event receiver
    pub fn log_receiver(&self) -> broadcast::Receiver<Log> {
        self.log_broadcast.subscribe()
    }

    /// Asynchronously returns all processes
    pub async fn processes(&self) -> Result<Vec<process::Handle>, Error> {
        let (sender, receiver) = oneshot::channel();
        let _ = self.sender.send(Request::Processes(sender)).await;
        Ok(receiver.await?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bpmn::schema::*;

    #[tokio::test]
    async fn spawn_and_terminate() {
        let model = Model::new(Default::default());
        let handle = model.spawn().await;
        handle.terminate().await;
    }

    #[tokio::test]
    async fn list_processes() {
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());
        let mut proc2: Process = Default::default();
        proc2.id = Some("proc2".into());

        let mut definitions: Definitions = Default::default();
        definitions.root_elements.push(RootElement::Process(proc1));
        definitions.root_elements.push(RootElement::Process(proc2));

        let model = Model::new(definitions);
        let handle = model.spawn().await;

        let mut process_names: Vec<String> = handle
            .processes()
            .await
            .unwrap()
            .into_iter()
            .map(|proc| proc.element().id.as_ref().unwrap().clone())
            .collect();

        process_names.sort();

        assert_eq!(process_names, vec!["proc1", "proc2"]);
    }
}
