//! # Task helpers
//!

#[cfg(not(any(target_arch = "wasm32", feature = "wasm-executor")))]
pub(crate) use tokio::task::*;

#[cfg(any(target_arch = "wasm32", feature = "wasm-executor"))]
mod wasm {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use thiserror::Error;
    use tokio::sync::oneshot;

    pub(crate) struct JoinHandle<T>(oneshot::Receiver<T>)
    where
        T: Send + 'static;

    impl<T> Future for JoinHandle<T>
    where
        T: Send + 'static,
    {
        type Output = Result<T, JoinError>;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.0.try_recv() {
                Ok(val) => Poll::Ready(Ok(val)),
                Err(oneshot::error::TryRecvError::Empty) => Poll::Pending,
                Err(_) => Poll::Ready(Err(JoinError)),
            }
        }
    }

    // TODO: make this error match tokio::task::JoinError (at least somewhat)
    #[derive(Error, Debug)]
    #[error("join error")]
    pub(crate) struct JoinError;

    #[allow(dead_code)]
    impl JoinError {
        pub(crate) fn is_cancelled(&self) -> bool {
            false
        }
        pub(crate) fn is_panic(&self) -> bool {
            true
        }
    }

    pub(crate) fn spawn<T>(task: T) -> JoinHandle<T::Output>
    where
        T: Future + 'static,
        T::Output: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();
        let _ = wasm_rs_async_executor::single_threaded::spawn(async move {
            let _ = sender.send(task.await);
        });
        JoinHandle(receiver)
    }

    pub(crate) fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // TODO: we need to do this in a worker
        let (sender, receiver) = oneshot::channel();
        let _ = sender.send(f());
        JoinHandle(receiver)
    }

    pub use tokio::task::yield_now;

    #[cfg(test)]
    use bpxe_internal_macros as bpxe_im;

    #[cfg(test)]
    #[bpxe_im::test]
    async fn spawn_and_join() {
        let handle = spawn(async move { 1 });
        assert_eq!(handle.await.unwrap(), 1);
    }
}

#[cfg(any(target_arch = "wasm32", feature = "wasm-executor"))]
pub(crate) use wasm::*;
