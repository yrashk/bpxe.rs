//! # Language expressions
//!
//! These are used for condition expressions and scripts
use crate::bpmn::schema::FormalExpression;
use async_trait::async_trait;
use std::collections::HashMap;
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
    async fn eval_expr(&mut self, expr: &Self::Expr) -> Result<T, EvaluationError>;
    /// Evaluates a program using an engine for a given language
    async fn eval(&mut self, code: &Self::Expr) -> Result<T, EvaluationError>;
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
    async fn eval_expr(&mut self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        let expr = expr.to_string();
        let engine = self.engine.clone();
        Ok(task::spawn_blocking(move || engine.eval_expression(&expr)).await??)
    }
    async fn eval(&mut self, expr: &Self::Expr) -> Result<T, EvaluationError> {
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

pub struct MultiLanguageEngine<T> {
    languages: HashMap<String, Box<dyn Engine<T, Expr = str>>>,
    default_language: Option<String>,
}

#[cfg(feature = "rhai")]
pub(crate) const RHAI_URI: &str = "https://rhaiscript.github.io/";

impl<T> MultiLanguageEngine<T>
where
    T: Send + Sync + Clone + 'static,
{
    /// Creates an empty engine
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new engine with all builtin engines
    pub fn new_with_builtin_engines() -> Self {
        let mut engine = MultiLanguageEngine::new();
        #[cfg(feature = "rhai")]
        engine.register_language(RHAI_URI, Rhai::default());
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
    pub fn register_language<S, E>(&mut self, name: S, engine: E)
    where
        S: Into<String>,
        E: Engine<T, Expr = str> + 'static,
    {
        self.languages.insert(name.into(), Box::new(engine));
    }

    fn get_engine_mut(
        &mut self,
        expr: &FormalExpression,
    ) -> Result<&mut Box<dyn Engine<T, Expr = str>>, EvaluationError> {
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
        match self.languages.get_mut(language) {
            None => Err(EvaluationError::UnsupportedLanguage {
                language: language.to_string(),
            }),
            Some(engine) => Ok(engine),
        }
    }
}

impl<T> Default for MultiLanguageEngine<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn default() -> Self {
        Self {
            languages: HashMap::new(),
            default_language: None,
        }
    }
}

#[async_trait]
impl<T> Engine<T> for MultiLanguageEngine<T>
where
    T: Send + Sync + Clone + 'static,
{
    type Expr = FormalExpression;
    async fn eval_expr(&mut self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        match expr.content {
            None => Err(EvaluationError::Empty),
            Some(ref e) => {
                let engine = self.get_engine_mut(expr)?;
                engine.eval_expr(&e).await
            }
        }
    }
    async fn eval(&mut self, expr: &Self::Expr) -> Result<T, EvaluationError> {
        match expr.content {
            None => Err(EvaluationError::Empty),
            Some(ref e) => {
                let engine = self.get_engine_mut(expr)?;
                engine.eval_expr(&e).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bpmn::schema::FormalExpression;

    #[cfg(test)]
    #[tokio::test]
    async fn unsupported_language() {
        let mut e = MultiLanguageEngine::<bool>::new();
        assert!(matches!(e.eval_expr(&FormalExpression {
            content: Some("true".into()),
            ..Default::default()
        }).await.unwrap_err(),
        EvaluationError::UnsupportedLanguage { language } if language == ""));
    }

    #[cfg(test)]
    #[tokio::test]
    async fn empty() {
        let mut e = MultiLanguageEngine::<bool>::new();
        assert!(matches!(
            e.eval_expr(&FormalExpression {
                ..Default::default()
            })
            .await
            .unwrap_err(),
            EvaluationError::Empty
        ));
    }

    #[cfg(all(test, feature = "rhai"))]
    #[tokio::test]
    async fn dispatch_to_rhai_evaluation() {
        let mut e = MultiLanguageEngine::<bool>::new_with_builtin_engines();
        assert!(e
            .eval_expr(&FormalExpression {
                language: Some(RHAI_URI.to_string()),
                content: Some("true".into()),
                ..Default::default()
            })
            .await
            .unwrap());
    }

    #[cfg(all(test, feature = "rhai"))]
    #[tokio::test]
    async fn rhai_return_type_mismatch() {
        let mut e: Box<dyn Engine<bool, Expr = str>> = Box::new(Rhai::new());
        assert!(matches!(e.eval_expr("3").await.unwrap_err(),
                 EvaluationError::ResultTypeError { expected, got } if expected == "bool" && got == "i64"));
    }

    #[cfg(all(test, feature = "rhai"))]
    #[tokio::test]
    async fn rhai_not_expr() {
        let mut e: Box<dyn Engine<bool, Expr = str>> = Box::new(Rhai::new());
        assert!(matches!(
                e.eval_expr("a = true")
                .await
                .unwrap_err(),
            EvaluationError::EvaluationError { .. }
        ));
    }

    #[cfg(all(test, feature = "rhai"))]
    #[tokio::test]
    async fn rhai_eval_program() {
        let mut e: Box<dyn Engine<bool, Expr = str>> = Box::new(Rhai::new());
        assert!(e.eval("let a = false; !a ").await.unwrap());
    }
}
