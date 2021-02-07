//! # Rhai Language support
//!
//! These are used for condition expressions and scripts
use super::{Engine, EngineContext, EngineContextProvider, EngineInfo, EvaluationError};
use crate::bpmn::schema::{FormalExpression, Script, ScriptTask};
use crate::data_object::{self, DataObject};
use crate::sys::task;
use async_trait::async_trait;
use num_bigint::BigInt;
use rhai::{Dynamic, RegisterFn, RegisterResultFn};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use thiserror::Error;

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

#[derive(Error, Debug)]
#[error("mutex error")]
struct MutexError;

impl Rhai {
    /// Tries to get a mutable reference to the engine
    ///
    /// Primarily useful for custom setup, tests, etc.
    pub fn engine_mut(&mut self) -> Option<&mut rhai::Engine> {
        Arc::get_mut(&mut self.engine)
    }

    #[inline]
    fn internal_eval<T>(&self, expr: &str, context: Context) -> Result<T, Box<rhai::EvalAltResult>>
    where
        T: Clone + Send + Sync + 'static,
    {
        type_translation!(self.engine.eval_with_scope(
            &mut *context
                .0
                .lock()
                .map_err(|_| rhai::EvalAltResult::ErrorSystem(
                    "mutex error".to_owned(),
                    Box::new(MutexError)
                ))?,
            expr
        ))
    }

    #[inline]
    fn internal_eval_expression<T>(
        &self,
        expr: &str,
        context: Context,
    ) -> Result<T, Box<rhai::EvalAltResult>>
    where
        T: Clone + Send + Sync + 'static,
    {
        type_translation!(self.engine.eval_expression_with_scope(
            &mut *context
                .0
                .lock()
                .map_err(|_| rhai::EvalAltResult::ErrorSystem(
                    "mutex error".to_owned(),
                    Box::new(MutexError)
                ))?,
            expr
        ))
    }

    fn unveil(value: &mut Box<dyn DataObject>) -> Result<Dynamic, Box<rhai::EvalAltResult>> {
        let value = dyn_clone::clone_box(value);
        if let Some(data_object::Container(v)) = value.downcast_ref::<data_object::Container<u8>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<i8>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<u16>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<i16>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<u32>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<i32>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<u64>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<i64>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<usize>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Container(v)) =
            value.downcast_ref::<data_object::Container<isize>>()
        {
            Ok(Dynamic::from(*v))
        } else if let Some(data_object::Empty) = value.downcast_ref::<data_object::Empty>() {
            Ok(Dynamic::UNIT)
        } else if let Some(json) = value.downcast_ref::<serde_json::Value>() {
            rhai::serde::to_dynamic(json)
        } else if let Ok(data_object::Collection(mut c)) =
            value.downcast::<data_object::Collection>().map(|v| *v)
        {
            let mut arr = vec![];
            for mut v in c.drain(..) {
                let item = Rhai::unveil(&mut v)?;
                arr.push(item);
            }
            Ok(Dynamic::from(arr))
        } else {
            Err("unsupported type".into())
        }
    }

    fn data_object<T: Send + Sync + Clone + Into<Dynamic> + 'static>(
        value: T,
    ) -> Box<dyn DataObject> {
        Box::new(data_object::Container(value))
    }

    fn output(array: rhai::Array) -> Result<Dynamic, Box<rhai::EvalAltResult>> {
        let mut result = Vec::new();
        for element in array {
            let type_name = element.type_name().to_string();
            if let Some(data_object) = element.try_cast::<Box<dyn DataObject>>() {
                result.push(data_object);
            } else {
                return Err(format!("unsupported type {}, expected DataObject", type_name).into());
            }
        }
        Ok(Dynamic::from(result))
    }
}

impl Default for Rhai {
    fn default() -> Self {
        let mut engine = rhai::Engine::new();
        engine
            .register_type_with_name::<Box<dyn DataObject>>("DataObject")
            .register_fn("data_object", Rhai::data_object::<rhai::INT>)
            .register_fn("data_object", Rhai::data_object::<rhai::FLOAT>)
            .register_fn("data_object", Rhai::data_object::<char>)
            .register_fn("data_object", Rhai::data_object::<bool>)
            .register_fn("data_object", Rhai::data_object::<()>)
            .register_fn("data_object", Rhai::data_object::<Dynamic>)
            .register_fn("data_object", Rhai::data_object::<instant::Instant>)
            .register_fn("data_object", Rhai::data_object::<rhai::Array>)
            .register_fn("data_object", Rhai::data_object::<rhai::Map>)
            .register_fn("data_object", Rhai::data_object::<rhai::ImmutableString>)
            .register_result_fn("unveil", Rhai::unveil)
            .register_type_with_name::<Vec<Box<dyn DataObject>>>("Output")
            .register_result_fn("output", Rhai::output);

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

pub(crate) const RHAI_URI: &str = "https://rhai.rs";
pub(crate) const RHAI_MIME: &str = "text/x-rhai";

impl EngineInfo for Rhai {
    fn namespace(&self) -> Option<&str> {
        Some(RHAI_URI)
    }

    fn mime_type(&self) -> Option<&str> {
        Some(RHAI_MIME)
    }
}

#[derive(Clone)]
pub struct Context(Arc<Mutex<rhai::Scope<'static>>>);

impl EngineContext for Context {
    fn clear(&mut self) -> &mut Self {
        if let Ok(mut scope) = self.0.lock() {
            scope.clear();
        }
        self
    }

    fn set(&mut self, name: &str, value: Box<dyn DataObject>) -> &mut Self {
        if let Ok(mut scope) = self.0.lock() {
            scope.push_dynamic(name.to_owned(), Dynamic::from(value));
        }
        self
    }
}

impl EngineContextProvider for Rhai {
    type Context = Context;

    fn new_context(&self) -> Self::Context {
        Context(Arc::new(Mutex::new(rhai::Scope::new())))
    }
}

#[async_trait]
impl Engine<FormalExpression> for Rhai {
    async fn eval<T>(
        &self,
        expr: &FormalExpression,
        context: &mut <Self as EngineContextProvider>::Context,
    ) -> Result<T, EvaluationError>
    where
        T: Send + Sync + Clone + 'static,
    {
        match expr.content {
            None => return Err(EvaluationError::Empty),
            Some(ref content) => {
                let expr = content.to_owned();
                let engine = self.clone();
                let context = context.clone();
                Ok(
                    task::spawn_blocking(move || engine.internal_eval_expression(&expr, context))
                        .await??,
                )
            }
        }
    }
}

#[async_trait]
impl Engine<ScriptTask> for Rhai {
    async fn eval<T>(
        &self,
        expr: &ScriptTask,
        context: &mut <Self as EngineContextProvider>::Context,
    ) -> Result<T, EvaluationError>
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
                let context = context.clone();
                Ok(task::spawn_blocking(move || engine.internal_eval(&expr, context)).await??)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::*;
    use ::rhai::Dynamic;
    use bpxe_internal_macros as bpxe_im;

    #[bpxe_im::test]
    async fn rhai_return_type_mismatch() {
        let e = Rhai::new();
        assert!(
            matches!(e.eval::<bool>(&FormalExpression { content: Some("3".into()), ..Default::default() }, &mut e.new_context()).await.unwrap_err(),
                 EvaluationError::ResultTypeError { expected, got } if expected == "bool" && got == "i64")
        );
    }

    #[bpxe_im::test]
    async fn rhai_not_expr() {
        let e = Rhai::new();
        assert!(matches!(
                e.eval::<bool>(&FormalExpression { content: Some("a = true".into()), ..Default::default() }, &mut e.new_context())
                .await
                .unwrap_err(),
            EvaluationError::EvaluationError { .. }
        ));
    }

    #[bpxe_im::test]
    async fn rhai_eval_program() {
        let e = Rhai::new();
        assert!(e
            .eval::<bool>(
                &ScriptTask {
                    script: Some(Script {
                        content: Some("let a = false; !a".into())
                    }),
                    ..Default::default()
                },
                &mut e.new_context()
            )
            .await
            .unwrap());
    }

    #[bpxe_im::test]
    async fn rhai_bigint_support() {
        use num_bigint::BigInt;
        let e = Rhai::new();
        assert_eq!(
            e.eval::<BigInt>(
                &ScriptTask {
                    script: Some(Script {
                        content: Some("100".into())
                    }),
                    ..Default::default()
                },
                &mut e.new_context()
            )
            .await
            .unwrap(),
            BigInt::from(100)
        );
    }

    #[bpxe_im::test]
    async fn rhai_bigint_support_expr() {
        use num_bigint::BigInt;
        let e = Rhai::new();
        assert_eq!(
            e.eval::<BigInt>(
                &FormalExpression {
                    content: Some("100".into()),
                    ..Default::default()
                },
                &mut e.new_context()
            )
            .await
            .unwrap(),
            BigInt::from(100)
        );
    }

    #[bpxe_im::test]
    async fn rhai_usize_support() {
        let e = Rhai::new();
        assert_eq!(
            e.eval::<usize>(
                &ScriptTask {
                    script: Some(Script {
                        content: Some("100".into())
                    },),
                    ..Default::default()
                },
                &mut e.new_context()
            )
            .await
            .unwrap(),
            100
        );
    }

    #[bpxe_im::test]
    async fn rhai_usize_support_expr() {
        let e = Rhai::new();
        assert_eq!(
            e.eval::<usize>(
                &FormalExpression {
                    content: Some("100".into()),
                    ..Default::default()
                },
                &mut e.new_context()
            )
            .await
            .unwrap(),
            100
        );
    }

    #[bpxe_im::test]
    async fn rhai_data_object_type() {
        let e = Rhai::new();
        let mut context = e.new_context();
        context.set("val", Box::new(crate::data_object::Container(100)));
        assert_eq!(
            e.eval::<String>(
                &FormalExpression {
                    content: Some("val.type_of()".into()),
                    ..Default::default()
                },
                &mut context
            )
            .await
            .unwrap(),
            "DataObject".to_string()
        );
    }

    #[bpxe_im::test]
    async fn rhai_unveil_primitive() {
        let e = Rhai::new();
        let mut context = e.new_context();
        context.set("val", Box::new(crate::data_object::Container(100)));
        assert_eq!(
            e.eval::<i32>(
                &FormalExpression {
                    content: Some("val.unveil()".into()),
                    ..Default::default()
                },
                &mut context
            )
            .await
            .unwrap(),
            100
        );
    }

    #[bpxe_im::test]
    async fn rhai_unveil_empty() {
        let e = Rhai::new();
        let mut context = e.new_context();
        context.set("val", Box::new(crate::data_object::Empty));
        assert_eq!(
            e.eval::<Dynamic>(
                &FormalExpression {
                    content: Some("val.unveil()".into()),
                    ..Default::default()
                },
                &mut context
            )
            .await
            .unwrap()
            .type_name(),
            "()"
        );
    }

    #[bpxe_im::test]
    async fn rhai_unveil_collection() {
        let e = Rhai::new();
        let mut context = e.new_context();
        context.set(
            "val",
            Box::new(crate::data_object::Collection(vec![Box::new(
                crate::data_object::Container(100),
            )])),
        );
        assert_eq!(
            e.eval::<i32>(
                &FormalExpression {
                    content: Some("val.unveil()[0]".into()),
                    ..Default::default()
                },
                &mut context
            )
            .await
            .unwrap(),
            100
        );
    }

    #[bpxe_im::test]
    async fn rhai_unveil_json() {
        let e = Rhai::new();
        let mut context = e.new_context();
        context.set("val", Box::new(serde_json::Value::Bool(true)));
        assert_eq!(
            e.eval::<bool>(
                &FormalExpression {
                    content: Some("val.unveil()".into()),
                    ..Default::default()
                },
                &mut context
            )
            .await
            .unwrap(),
            true
        );
    }

    #[bpxe_im::test]
    async fn rhai_data_object() {
        // TODO: this is not a full coverage of supported types
        let e = Rhai::new();
        let mut context = e.new_context();
        assert_eq!(
            e.eval::<i64>(
                &FormalExpression {
                    content: Some("data_object(100).unveil()".into()),
                    ..Default::default()
                },
                &mut context
            )
            .await
            .unwrap(),
            100
        );
    }
}
