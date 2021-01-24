//! # Language expressions
//!
//! These are used for condition expressions and scripts
use crate::bpmn::schema::{FormalExpression, Script, ScriptTask};
use async_trait::async_trait;
use std::collections::HashMap;
use std::marker::PhantomData;
#[cfg(feature = "rhai")]
use std::ops::Deref;
#[cfg(feature = "rhai")]
use std::sync::Arc;
use thiserror::Error;
use tokio::task;

/// Language for expressions and/or scripts
#[async_trait]
pub trait Engine<T>: Send + Sync
where
    T: Send + Sync + Clone + 'static,
{
    /// Types of accepted expressions
    type Expr: ?Sized;
    /// Evaluates an expression using an engine for a given language
    async fn eval_expr(&self, expr: &Self::Expr) -> Result<T, EvaluationError>;
    /// Evaluates a program using an engine for a given language
    async fn eval(&self, code: &Self::Expr) -> Result<T, EvaluationError>;
}

/// Rhai language engine
#[cfg(feature = "rhai")]
pub struct Rhai {
    engine: Arc<rhai::Engine>,
}

#[cfg(feature = "rhai")]
impl Rhai {
    /// Creates a new Rhai engine
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(feature = "rhai")]
impl Deref for Rhai {
    type Target = rhai::Engine;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

#[cfg(feature = "rhai")]
impl Rhai {
    /// Tries to get a mutable reference to the engine
    ///
    /// Primarily useful for custom setup, tests, etc.
    pub fn engine_mut(&mut self) -> Option<&mut rhai::Engine> {
        Arc::get_mut(&mut self.engine)
    }
}

#[cfg(feature = "rhai")]
impl Default for Rhai {
    fn default() -> Self {
        let engine = rhai::Engine::new();
        Self {
            engine: Arc::new(engine),
        }
    }
}

#[cfg(feature = "rhai")]
impl From<Box<rhai::EvalAltResult>> for EvaluationError {
    fn from(e: Box<rhai::EvalAltResult>) -> Self {
        match *e {
            rhai::EvalAltResult::ErrorMismatchOutputType(expected, got, _) => {
                EvaluationError::ResultTypeError { expected, got }
            }
            e => EvaluationError::EvaluationError { error: Box::new(e) },
        }
    }
}

#[cfg(feature = "rhai")]
#[async_trait]
impl<T> Engine<T> for Rhai
where
    T: Send + Sync + Clone + 'static,
{
    type Expr = str;
    async fn eval_expr(&self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        let expr = expr.to_string();
        let engine = self.engine.clone();
        Ok(task::spawn_blocking(move || engine.eval_expression(&expr)).await??)
    }
    async fn eval(&self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        let expr = expr.to_string();
        let engine = self.engine.clone();
        Ok(task::spawn_blocking(move || engine.eval(&expr)).await??)
    }
}

/// Evaluation error
#[derive(Error, Debug)]
pub enum EvaluationError {
    /// Given language is not supported
    #[error("unsupported language {language}")]
    UnsupportedLanguage { language: String },
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

pub struct MultiLanguageEngine<T, E> {
    languages: HashMap<String, Box<dyn Engine<T, Expr = str>>>,
    default_language: Option<String>,
    phantom_data: PhantomData<E>,
}

#[cfg(feature = "rhai")]
pub(crate) const RHAI_URI: &str = "https://rhaiscript.github.io/";

#[cfg(feature = "rhai")]
pub(crate) const RHAI_MIME: &str = "text/x-rhai";

impl<T, E> MultiLanguageEngine<T, E>
where
    T: Send + Sync + Clone + 'static,
{
    /// Creates an empty engine
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new engine with all builtin engines
    pub fn new_with_builtin_engines() -> Self {
        #[allow(unused_mut)]
        let mut engine = MultiLanguageEngine::new();
        #[cfg(feature = "rhai")]
        {
            engine.register_language(RHAI_URI, Rhai::default());
            engine.register_language(RHAI_MIME, Rhai::default());
        }
        #[cfg(feature = "rhai")]
        engine.set_default_language(RHAI_URI);
        engine
    }

    /// Sets a default engine
    ///
    /// Currently, by default, Rhai is the default language (unless `rhai` feature is disabled)
    pub fn set_default_language<S>(&mut self, s: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.default_language = Some(s.into());
        self
    }

    /// Registers a language engine
    pub fn register_language<S, Eng>(&mut self, name: S, engine: Eng)
    where
        S: Into<String>,
        Eng: Engine<T, Expr = str> + 'static,
    {
        self.languages.insert(name.into(), Box::new(engine));
    }
}

impl<T, E> Default for MultiLanguageEngine<T, E>
where
    T: Send + Sync + Clone + 'static,
{
    fn default() -> Self {
        Self {
            languages: HashMap::new(),
            default_language: None,
            phantom_data: PhantomData,
        }
    }
}

trait EngineRetrieval<T, Type>
where
    Type: Send + Sync + 'static,
{
    #[allow(clippy::borrowed_box)]
    fn get_engine(&self, item: &T) -> Result<&Box<dyn Engine<Type, Expr = str>>, EvaluationError>;
}

impl<T> EngineRetrieval<FormalExpression, T> for MultiLanguageEngine<T, FormalExpression>
where
    T: Send + Sync + 'static,
{
    fn get_engine(
        &self,
        expr: &FormalExpression,
    ) -> Result<&Box<dyn Engine<T, Expr = str>>, EvaluationError> {
        let language = match expr.language {
            None => match self.default_language {
                None => {
                    return Err(EvaluationError::UnsupportedLanguage {
                        language: "".into(),
                    })
                }
                Some(ref language) => language,
            },
            Some(ref uri) => uri,
        };
        match self.languages.get(language) {
            None => Err(EvaluationError::UnsupportedLanguage {
                language: language.to_string(),
            }),
            Some(engine) => Ok(engine),
        }
    }
}

impl<T> EngineRetrieval<ScriptTask, T> for MultiLanguageEngine<T, ScriptTask>
where
    T: Send + Sync + 'static,
{
    fn get_engine(
        &self,
        script: &ScriptTask,
    ) -> Result<&Box<dyn Engine<T, Expr = str>>, EvaluationError> {
        let language = match script.script_format {
            None => match self.default_language {
                None => {
                    return Err(EvaluationError::UnsupportedLanguage {
                        language: "".into(),
                    })
                }
                Some(ref language) => language,
            },
            Some(ref mime_type) => mime_type,
        };
        match self.languages.get(language) {
            None => Err(EvaluationError::UnsupportedLanguage {
                language: language.to_string(),
            }),
            Some(engine) => Ok(engine),
        }
    }
}

#[async_trait]
impl<T> Engine<T> for MultiLanguageEngine<T, FormalExpression>
where
    T: Send + Sync + Clone + 'static,
{
    type Expr = FormalExpression;
    async fn eval_expr(&self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        match expr.content {
            None => Err(EvaluationError::Empty),
            Some(ref e) => {
                let engine = self.get_engine(expr)?;
                engine.eval_expr(&e).await
            }
        }
    }
    async fn eval(&self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        match expr.content {
            None => Err(EvaluationError::Empty),
            Some(ref e) => {
                let engine = self.get_engine(expr)?;
                engine.eval(&e).await
            }
        }
    }
}

#[async_trait]
impl Engine<()> for MultiLanguageEngine<(), ScriptTask> {
    type Expr = ScriptTask;
    async fn eval_expr(&self, script: &Self::Expr) -> Result<(), EvaluationError> {
        match script.script {
            None => Err(EvaluationError::Empty),
            Some(Script { content: None }) => Err(EvaluationError::Empty),
            Some(Script {
                content: Some(ref e),
            }) => {
                let engine = self.get_engine(script)?;
                match engine.eval_expr(&e).await {
                    Err(EvaluationError::ResultTypeError { .. }) => Ok(()),
                    other => other.map(|_| ()),
                }
            }
        }
    }
    async fn eval(&self, script: &Self::Expr) -> Result<(), EvaluationError> {
        match script.script {
            None => Err(EvaluationError::Empty),
            Some(Script { content: None }) => Err(EvaluationError::Empty),
            Some(Script {
                content: Some(ref e),
            }) => {
                let engine = self.get_engine(script)?;
                match engine.eval(&e).await {
                    Err(EvaluationError::ResultTypeError { .. }) => Ok(()),
                    other => other.map(|_| ()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unsupported_language() {
        let e = MultiLanguageEngine::<bool, FormalExpression>::new();
        assert!(matches!(e.eval_expr(&FormalExpression {
            content: Some("true".into()),
            ..Default::default()
        }).await.unwrap_err(),
        EvaluationError::UnsupportedLanguage { language } if language == ""));
    }

    #[tokio::test]
    async fn empty() {
        let e = MultiLanguageEngine::<bool, FormalExpression>::new();
        assert!(matches!(
            e.eval_expr(&FormalExpression {
                ..Default::default()
            })
            .await
            .unwrap_err(),
            EvaluationError::Empty
        ));
    }

    #[tokio::test]
    async fn script_task_empty() {
        let e = MultiLanguageEngine::<(), ScriptTask>::new();
        assert!(matches!(
            e.eval_expr(&ScriptTask {
                script: None,
                ..Default::default()
            })
            .await
            .unwrap_err(),
            EvaluationError::Empty
        ));
        assert!(matches!(
            e.eval_expr(&ScriptTask {
                script: Some(Script { content: None }),
                ..Default::default()
            })
            .await
            .unwrap_err(),
            EvaluationError::Empty
        ));
    }

    #[tokio::test]
    async fn script_task() {
        let e = MultiLanguageEngine::<(), ScriptTask>::new_with_builtin_engines();
        assert!(e
            .eval_expr(&ScriptTask {
                script: Some(Script {
                    content: Some("true".into())
                }),
                ..Default::default()
            })
            .await
            .is_ok());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn dispatch_to_rhai_evaluation() {
        let e = MultiLanguageEngine::<bool, FormalExpression>::new_with_builtin_engines();
        assert!(e
            .eval_expr(&FormalExpression {
                language: Some(RHAI_URI.to_string()),
                content: Some("true".into()),
                ..Default::default()
            })
            .await
            .unwrap());
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn rhai_return_type_mismatch() {
        let e: Box<dyn Engine<bool, Expr = str>> = Box::new(Rhai::new());
        assert!(matches!(e.eval_expr("3").await.unwrap_err(),
                 EvaluationError::ResultTypeError { expected, got } if expected == "bool" && got == "i64"));
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn rhai_not_expr() {
        let e: Box<dyn Engine<bool, Expr = str>> = Box::new(Rhai::new());
        assert!(matches!(
                e.eval_expr("a = true")
                .await
                .unwrap_err(),
            EvaluationError::EvaluationError { .. }
        ));
    }

    #[cfg(feature = "rhai")]
    #[tokio::test]
    async fn rhai_eval_program() {
        let e: Box<dyn Engine<bool, Expr = str>> = Box::new(Rhai::new());
        assert!(e.eval("let a = false; !a ").await.unwrap());
    }
}
