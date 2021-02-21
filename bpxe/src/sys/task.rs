//! # Task helpers
//!

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use tokio::task::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::future::Future;
    use wasm_rs_async_executor::single_threaded as executor;

    pub type JoinHandle<T> = executor::TaskHandle<T>;
    pub type JoinError = executor::JoinError;

    pub(crate) fn spawn<T>(task: T) -> executor::TaskHandle<T::Output>
    where
        T: Future + 'static,
        T::Output: Send + 'static,
    {
        executor::spawn(task)
    }

    pub(crate) fn spawn_blocking<F, R>(f: F) -> executor::TaskHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // TODO: we need to do this in a worker
        executor::spawn(async { f() })
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

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::*;
