//! # Language expressions;
//!
//! These are used for condition expressions and scripts
use crate::bpmn::schema::{FormalExpression, ScriptTask};
use crate::data_object::DataObject;
use crate::sys::task;
use async_trait::async_trait;
use thiserror::Error;

#[cfg(feature = "rhai")]
pub mod rhai;

/// Information about the engine
pub trait EngineInfo {
    /// Returns language's namespace
    fn namespace(&self) -> Option<&str> {
        None
    }
    /// Returns language's mime type
    fn mime_type(&self) -> Option<&str> {
        None
    }
}

/// Engine context provider
pub trait EngineContextProvider {
    /// Context type
    type Context: EngineContext;

    /// Returns a newly initialized context
    fn new_context(&self) -> Self::Context;
}

/// Engine execution context
pub trait EngineContext {
    /// Clears the context
    fn clear(&mut self) -> &mut Self;

    /// Sets a value in the context
    fn set(&mut self, name: &str, value: Box<dyn DataObject>) -> &mut Self;
}

/// Language for expressions and/or scripts
#[async_trait]
pub trait Engine<Expr>: EngineInfo + EngineContextProvider + Send + Sync
where
    Expr: Send + Sync + Unpin + 'static,
{
    /// Evaluates a program using an engine for a given language
    async fn eval<T>(
        &self,
        code: &Expr,
        context: &mut <Self as EngineContextProvider>::Context,
    ) -> Result<T, EvaluationError>
    where
        T: Send + Sync + Clone + 'static;
}

/// Evaluation error
#[derive(Error, Debug)]
pub enum EvaluationError {
    /// Given language is not supported
    #[error("unsupported language {language:?}")]
    UnsupportedLanguage { language: Option<String> },
    #[error("evaluation error {error:?}")]
    /// Error during evaluation
    EvaluationError {
        #[from]
        error: Box<dyn std::error::Error + Send>,
    },
    /// Result type mismatch
    #[error("result type mismatch, expected {expected}, got {got}")]
    ResultTypeError { expected: String, got: String },
    /// Empty expression or script
    #[error("empty expression or script")]
    Empty,
    #[error("execution error {0:?}")]
    ExecutionError(ExecutionError),
}

#[derive(Debug)]
pub enum ExecutionError {
    Cancelled,
    Panicked,
    UnknownCause,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<task::JoinError> for EvaluationError {
    fn from(err: task::JoinError) -> Self {
        if err.is_cancelled() {
            return Self::ExecutionError(ExecutionError::Cancelled);
        }
        if err.is_panic() {
            return Self::ExecutionError(ExecutionError::Panicked);
        }
        Self::ExecutionError(ExecutionError::UnknownCause)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<task::JoinError> for EvaluationError {
    fn from(_err: task::JoinError) -> Self {
        Self::ExecutionError(ExecutionError::UnknownCause)
    }
}

pub struct MultiLanguageEngine {
    default_namespace: Option<String>,
    default_mime_type: Option<String>,
    #[cfg(feature = "rhai")]
    pub rhai: rhai::Rhai,
}

impl MultiLanguageEngine {
    /// Creates a new default engine
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "rhai"))]
            default_namespace: None,
            #[cfg(feature = "rhai")]
            default_namespace: Some(rhai::RHAI_URI.to_owned()),
            #[cfg(not(feature = "rhai"))]
            default_mime_type: None,
            #[cfg(feature = "rhai")]
            default_mime_type: Some(rhai::RHAI_MIME.to_owned()),
            #[cfg(feature = "rhai")]
            rhai: Default::default(),
        }
    }

    /// Sets default namespace
    pub fn set_default_namespace<S: Into<String>>(&mut self, ns: S) {
        self.default_namespace.replace(ns.into());
    }

    /// Sets default mime type
    pub fn set_default_mime_type<S: Into<String>>(&mut self, mime_type: S) {
        self.default_mime_type.replace(mime_type.into());
    }
}

impl EngineInfo for MultiLanguageEngine {}

#[async_trait]
impl Engine<FormalExpression> for MultiLanguageEngine {
    async fn eval<T>(
        &self,
        code: &FormalExpression,
        context: &mut <Self as EngineContextProvider>::Context,
    ) -> Result<T, EvaluationError>
    where
        T: Send + Sync + Clone + 'static,
    {
        let language = match code.language.as_ref() {
            None => self.default_namespace.as_ref(),
            Some(language) => Some(language),
        };
        match language {
            None => return Err(EvaluationError::UnsupportedLanguage { language: None }),
            Some(language) => {
                #[cfg(feature = "rhai")]
                if let Some(ns) = self.rhai.namespace() {
                    if ns == language {
                        return self.rhai.eval(code, &mut context.rhai_context).await;
                    }
                }
                return Err(EvaluationError::UnsupportedLanguage {
                    language: Some(language.into()),
                });
            }
        }
    }
}

#[async_trait]
impl Engine<ScriptTask> for MultiLanguageEngine {
    async fn eval<T>(
        &self,
        code: &ScriptTask,
        context: &mut <Self as EngineContextProvider>::Context,
    ) -> Result<T, EvaluationError>
    where
        T: Send + Sync + Clone + 'static,
    {
        let language = match code.script_format.as_ref() {
            None => self.default_mime_type.as_ref(),
            Some(language) => Some(language),
        };
        match language {
            None => return Err(EvaluationError::UnsupportedLanguage { language: None }),
            Some(language) => {
                #[cfg(feature = "rhai")]
                if let Some(t) = self.rhai.mime_type() {
                    if t == language {
                        return self.rhai.eval(code, &mut context.rhai_context).await;
                    }
                }
                return Err(EvaluationError::UnsupportedLanguage {
                    language: Some(language.into()),
                });
            }
        }
    }
}

pub struct MultiLanguageEngineContext {
    #[cfg(feature = "rhai")]
    rhai_context: rhai::Context,
}

impl EngineContext for MultiLanguageEngineContext {
    fn clear(&mut self) -> &mut Self {
        #[cfg(feature = "rhai")]
        self.rhai_context.clear();
        self
    }

    /// Sets a value in the context
    fn set(&mut self, name: &str, value: Box<dyn DataObject>) -> &mut Self {
        #[cfg(feature = "rhai")]
        self.rhai_context.set(name, dyn_clone::clone_box(&*value));
        self
    }
}

impl EngineContextProvider for MultiLanguageEngine {
    type Context = MultiLanguageEngineContext;

    fn new_context(&self) -> Self::Context {
        MultiLanguageEngineContext {
            #[cfg(feature = "rhai")]
            rhai_context: self.rhai.new_context(),
        }
    }
}

impl Default for MultiLanguageEngine {
    fn default() -> Self {
        MultiLanguageEngine::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bpxe_internal_macros as bpxe_im;

    use crate::language::rhai::*;

    #[bpxe_im::test]
    async fn dispatch_to_rhai_evaluation() {
        let e = MultiLanguageEngine::new();
        assert!(e
            .eval::<bool>(
                &FormalExpression {
                    language: Some(RHAI_URI.to_string()),
                    content: Some("true".into()),
                    ..Default::default()
                },
                &mut e.new_context()
            )
            .await
            .unwrap());
    }
}
