//! # Language expressions
//!
//! These are used for condition expressions and scripts
use thiserror::Error;

const RHAI_URI: &str = "https://rhaiscript.github.io/";

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
        error: Box<dyn std::error::Error>,
    },
    /// Result type mismatch
    #[error("result type mismatch, expected {expected}, got {got}")]
    ResultTypeError { expected: String, got: String },
}

/// Build-time constructed multi-engine expression evaluation
pub struct ExpressionEvaluator {
    #[cfg(feature = "rhai")]
    rhai: rhai::Engine,
    default_engine: String,
}

impl ExpressionEvaluator {
    /// Creates a new ExpressionEvaluator
    pub fn new() -> Self {
        #[cfg(feature = "rhai")]
        let mut rhai = rhai::Engine::new();
        rhai.set_module_resolver(rhai::module_resolvers::DummyModuleResolver::new());
        Self {
            #[cfg(feature = "rhai")]
            rhai,
            #[cfg(feature = "rhai")]
            default_engine: RHAI_URI.into(),
            #[cfg(not(any(feature = "rhai")))]
            default_engine: "",
        }
    }

    /// Sets a default engine
    ///
    /// Currently, by default, Rhai is the default language (unless `rhai` feature is disabled)
    pub fn set_default_engine<S: Into<String>>(&mut self, s: S) -> &mut Self {
        self.default_engine = s.into();
        self
    }

    /// Gets instance of Rhai engine
    ///
    /// Use this to customize it
    #[cfg(feature = "rhai")]
    pub fn get_rhai(&mut self) -> &mut rhai::Engine {
        &mut self.rhai
    }

    /// Evaluates an expression using an engine for a given language
    ///
    /// If `language` is None, default engine will be used.
    pub fn eval_expr<T>(
        &self,
        language: Option<&String>,
        expression: &str,
    ) -> Result<T, EvaluationError>
    where
        T: Clone + Send + Sync + 'static,
    {
        let language = language.unwrap_or(&self.default_engine);
        #[cfg(feature = "rhai")]
        if language == RHAI_URI {
            return self.rhai.eval_expression(expression).map_err(|e| match *e {
                rhai::EvalAltResult::ErrorMismatchOutputType(expected, got, _) => {
                    EvaluationError::ResultTypeError { expected, got }
                }
                e => EvaluationError::EvaluationError { error: Box::new(e) },
            });
        }
        Err(EvaluationError::UnsupportedLanguage {
            language: language.into(),
        })
    }
}

impl Default for ExpressionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[test]
fn unsupported_language() {
    let e = ExpressionEvaluator::new();
    assert!(
        matches!(e.eval_expr::<bool>(Some(&"unknown".to_string()), "test").unwrap_err(),
        EvaluationError::UnsupportedLanguage { language } if language == "unknown")
    );
}

#[cfg(all(test, feature = "rhai"))]
#[test]
fn rhai_evaluation() {
    let e = ExpressionEvaluator::new();
    assert!(e
        .eval_expr::<bool>(Some(&RHAI_URI.to_string()), "true")
        .unwrap());
}

#[cfg(all(test, feature = "rhai"))]
#[test]
fn rhai_return_type_mismatch() {
    let e = ExpressionEvaluator::new();
    assert!(
        matches!(e.eval_expr::<bool>(Some(&RHAI_URI.to_string()), "3").unwrap_err(), 
                 EvaluationError::ResultTypeError { expected, got } if expected == "bool" && got == "i64")
    );
}

#[cfg(all(test, feature = "rhai"))]
#[test]
fn rhai_not_expr() {
    let e = ExpressionEvaluator::new();
    assert!(matches!(
        e.eval_expr::<bool>(Some(&RHAI_URI.to_string()), "a = true")
            .unwrap_err(),
        EvaluationError::EvaluationError { .. }
    ));
}
