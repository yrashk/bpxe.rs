//! # Rhai Language support
//!
//! These are used for condition expressions and scripts
use super::{Engine, EngineInfo, EvaluationError};
use crate::bpmn::schema::{FormalExpression, Script, ScriptTask};
use async_trait::async_trait;
use num_bigint::BigInt;
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;
use tokio::task;

/// Rhai language engine
#[derive(Clone)]
pub struct Rhai {
    engine: Arc<rhai::Engine>,
}

impl Rhai {
    /// Creates a new Rhai engine
    pub fn new() -> Self {
        Default::default()
    }
}

impl Deref for Rhai {
    type Target = rhai::Engine;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

// Translate types that Rhai doesn't support
//
// This is necessary to establish types that are shared between different languages
// so that the engine doesn't need to be aware of the language being used.
macro_rules! type_translation {
    ($expr: expr, $t: ty, $t1: ty, $ident: ident, $convert: expr) => {
        if TypeId::of::<T>() == TypeId::of::<$t1>() {
            let result: Result<$t, _> = $expr;
            return result.map(|$ident| {
                let v1 = $convert;
                let any: &dyn Any = &v1;
                any.downcast_ref::<T>().unwrap().clone()
            });
        }
    };
    ($expr: expr) => {{
        type_translation!($expr, i64, usize, value, value as usize);
        type_translation!($expr, i64, BigInt, value, BigInt::from(value));
        $expr
    }};
}

impl Rhai {
    /// Tries to get a mutable reference to the engine
    ///
    /// Primarily useful for custom setup, tests, etc.
    pub fn engine_mut(&mut self) -> Option<&mut rhai::Engine> {
        Arc::get_mut(&mut self.engine)
    }

    #[inline]
    fn internal_eval<T>(&self, expr: &str) -> Result<T, Box<rhai::EvalAltResult>>
    where
        T: Clone + Send + Sync + 'static,
    {
        type_translation!(self.engine.eval(expr))
    }

    #[inline]
    fn internal_eval_expression<T>(&self, expr: &str) -> Result<T, Box<rhai::EvalAltResult>>
    where
        T: Clone + Send + Sync + 'static,
    {
        type_translation!(self.engine.eval_expression(expr))
    }
}

impl Default for Rhai {
    fn default() -> Self {
        let engine = rhai::Engine::new();
        Self {
            engine: Arc::new(engine),
        }
    }
}

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

pub(crate) const RHAI_URI: &str = "https://rhaiscript.github.io/";
pub(crate) const RHAI_MIME: &str = "text/x-rhai";

impl EngineInfo for Rhai {
    fn namespace(&self) -> Option<&str> {
        Some(RHAI_URI)
    }

    fn mime_type(&self) -> Option<&str> {
        Some(RHAI_MIME)
    }
}

#[async_trait]
impl Engine<FormalExpression> for Rhai {
    async fn eval<T>(&self, expr: &FormalExpression) -> Result<T, EvaluationError>
    where
        T: Send + Sync + Clone + 'static,
    {
        match expr.content {
            None => return Err(EvaluationError::Empty),
            Some(ref content) => {
                let expr = content.to_owned();
                let engine = self.clone();
                Ok(task::spawn_blocking(move || engine.internal_eval_expression(&expr)).await??)
            }
        }
    }
}

#[async_trait]
impl Engine<ScriptTask> for Rhai {
    async fn eval<T>(&self, expr: &ScriptTask) -> Result<T, EvaluationError>
    where
        T: Send + Sync + Clone + 'static,
    {
        match expr.script {
            None | Some(Script { content: None }) => return Err(EvaluationError::Empty),
            Some(Script {
                content: Some(ref content),
            }) => {
                let expr = content.to_owned();
                let engine = self.clone();
                Ok(task::spawn_blocking(move || engine.internal_eval(&expr)).await??)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::*;

    #[tokio::test]
    async fn rhai_return_type_mismatch() {
        let e = Rhai::new();
        assert!(
            matches!(e.eval::<bool>(&FormalExpression { content: Some("3".into()), ..Default::default() }).await.unwrap_err(),
                 EvaluationError::ResultTypeError { expected, got } if expected == "bool" && got == "i64")
        );
    }

    #[tokio::test]
    async fn rhai_not_expr() {
        let e = Rhai::new();
        assert!(matches!(
                e.eval::<bool>(&FormalExpression { content: Some("a = true".into()), ..Default::default() })
                .await
                .unwrap_err(),
            EvaluationError::EvaluationError { .. }
        ));
    }

    #[tokio::test]
    async fn rhai_eval_program() {
        let e = Rhai::new();
        assert!(e
            .eval::<bool>(&ScriptTask {
                script: Some(Script {
                    content: Some("let a = false; !a".into())
                }),
                ..Default::default()
            })
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn rhai_bigint_support() {
        use num_bigint::BigInt;
        let e = Rhai::new();
        assert_eq!(
            e.eval::<BigInt>(&ScriptTask {
                script: Some(Script {
                    content: Some("100".into())
                }),
                ..Default::default()
            })
            .await
            .unwrap(),
            BigInt::from(100)
        );
    }

    #[tokio::test]
    async fn rhai_bigint_support_expr() {
        use num_bigint::BigInt;
        let e = Rhai::new();
        assert_eq!(
            e.eval::<BigInt>(&FormalExpression {
                content: Some("100".into()),
                ..Default::default()
            })
            .await
            .unwrap(),
            BigInt::from(100)
        );
    }

    #[tokio::test]
    async fn rhai_usize_support() {
        let e = Rhai::new();
        assert_eq!(
            e.eval::<usize>(&ScriptTask {
                script: Some(Script {
                    content: Some("100".into())
                }),
                ..Default::default()
            })
            .await
            .unwrap(),
            100
        );
    }

    #[tokio::test]
    async fn rhai_usize_support_expr() {
        let e = Rhai::new();
        assert_eq!(
            e.eval::<usize>(&FormalExpression {
                content: Some("100".into()),
                ..Default::default()
            })
            .await
            .unwrap(),
            100
        );
    }
}
