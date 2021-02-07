//! # Model
//!
//! Model is a central entrypoint to BPMN execution. It contains all definitions of a BPMN document
//! and orchestrates process instantiation and execution.
use crate::bpmn::schema::{Definitions, RootElement};
use crate::language::MultiLanguageEngine;
use crate::process;
use factory::Factory;
use futures::future::join_all;

use crate::sys::task::{self, JoinHandle};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{broadcast, mpsc, oneshot};

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

/// Script engine factory
pub trait LanguageEngineFactory: Factory<Item = MultiLanguageEngine> + Send + Sync {}

/// Function/closure script engine factory
pub struct FnLanguageEngineFactory<F>(pub F)
where
    F: Fn() -> MultiLanguageEngine + Send + Sync;

impl<F> Factory for FnLanguageEngineFactory<F>
where
    F: Fn() -> MultiLanguageEngine + Send + Sync,
{
    type Item = MultiLanguageEngine;

    fn create(&self) -> Self::Item {
        self.0()
    }
}

impl<F> LanguageEngineFactory for FnLanguageEngineFactory<F> where
    F: Fn() -> MultiLanguageEngine + Send + Sync
{
}

/// Default script engine factory
pub struct DefaultLanguageEngineFactory;

impl Factory for DefaultLanguageEngineFactory {
    type Item = MultiLanguageEngine;

    fn create(&self) -> Self::Item {
        MultiLanguageEngine::new()
    }
}

impl LanguageEngineFactory for DefaultLanguageEngineFactory {}

/// Expression engine factory
pub trait ExpressionEngineFactory<T>: Factory<Item = MultiLanguageEngine> + Send + Sync
where
    T: Send + Sync + 'static,
{
}

/// Model is a container for a BPMN document
pub struct Model<ScriptEngine, ExpressionEngine>
where
    ScriptEngine: LanguageEngineFactory,
    ExpressionEngine: LanguageEngineFactory,
{
    definitions: Arc<Definitions>,
    processes: Vec<process::Handle>,
    script_engine_factory: Option<ScriptEngine>,
    expression_engine_factory: Option<ExpressionEngine>,
}

/// Control handle for a running model
#[derive(Clone)]
pub struct Handle {
    definitions: Arc<Definitions>,
    sender: mpsc::Sender<Request>,
    log_broadcast: broadcast::Sender<Log>,
    script_engine_factory: Arc<Box<dyn LanguageEngineFactory>>,
    expression_engine_factory: Arc<Box<dyn LanguageEngineFactory>>,
}

/// Model events
#[derive(Clone, Debug)]
pub enum Log {}

enum Request {
    JoinHandle(JoinHandle<()>),
    Terminate(oneshot::Sender<Option<JoinHandle<()>>>),
    Processes(oneshot::Sender<Vec<process::Handle>>),
}

impl Model<DefaultLanguageEngineFactory, DefaultLanguageEngineFactory> {
    /// Initializes a model
    ///
    /// In order to make it operational, use [`Model::spawn`]
    pub fn new(definitions: Definitions) -> Self {
        Self {
            definitions: Arc::new(definitions),
            processes: vec![],
            script_engine_factory: Some(DefaultLanguageEngineFactory),
            expression_engine_factory: Some(DefaultLanguageEngineFactory),
        }
    }
}

impl<ScriptEngine, ExpressionEngine> Model<ScriptEngine, ExpressionEngine>
where
    ScriptEngine: LanguageEngineFactory + 'static,
    ExpressionEngine: LanguageEngineFactory + 'static,
{
    /// Consumes model and returns it updated with a new script engine factory
    pub fn with_script_engine_factory<Factory>(
        self,
        script_engine_factory: Factory,
    ) -> Model<Factory, ExpressionEngine>
    where
        Factory: LanguageEngineFactory,
    {
        Model {
            script_engine_factory: Some(script_engine_factory),
            expression_engine_factory: self.expression_engine_factory,
            definitions: self.definitions,
            processes: self.processes,
        }
    }

    /// Consumes model and returns it updated with a new expression engine factory
    pub fn with_expression_engine_factory<Factory>(
        self,
        expression_engine_factory: Factory,
    ) -> Model<ScriptEngine, Factory>
    where
        Factory: LanguageEngineFactory,
    {
        Model {
            script_engine_factory: self.script_engine_factory,
            expression_engine_factory: Some(expression_engine_factory),
            definitions: self.definitions,
            processes: self.processes,
        }
    }

    /// Spawns model operation task
    pub async fn spawn(mut self) -> Handle {
        let (sender, receiver) = mpsc::channel(1);
        let (log_broadcast, _) = broadcast::channel(128);
        let log_sender = log_broadcast.clone();
        let handle = Handle {
            definitions: self.definitions.clone(),
            sender: sender.clone(),
            log_broadcast,
            // these unwraps should be ok because we don't use `None` for `script_engine_factory`
            // and `script_expression_factory` anywhere
            script_engine_factory: Arc::new(Box::new(self.script_engine_factory.take().unwrap())),
            expression_engine_factory: Arc::new(Box::new(
                self.expression_engine_factory.take().unwrap(),
            )),
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
                    for process in self.processes.drain(..) {
                        let _ = process.terminate().await;
                    }
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

    /// Returns model's script engine factory
    pub fn script_engine_factory(&self) -> Arc<Box<dyn LanguageEngineFactory>> {
        self.script_engine_factory.clone()
    }

    /// Returns model's expression engine factory
    pub fn expression_engine_factory(&self) -> Arc<Box<dyn LanguageEngineFactory>> {
        self.expression_engine_factory.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bpmn::schema::*;
    use bpxe_internal_macros as bpxe_im;

    #[bpxe_im::test]
    async fn spawn_and_terminate() {
        let model = Model::new(Default::default());
        let handle = model.spawn().await;
        handle.terminate().await;
    }

    #[bpxe_im::test]
    async fn list_processes() {
        let definitions = Definitions {
            root_elements: vec![
                Process {
                    id: Some("proc1".into()),
                    ..Default::default()
                }
                .into(),
                Process {
                    id: Some("proc2".into()),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        };

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
