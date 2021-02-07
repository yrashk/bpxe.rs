//! # Testing Helpers
//!
mod mailbox;
pub(crate) use mailbox::Mailbox;

use crate::sys::task;
#[cfg(not(any(feature = "wasm-executor", target_arch = "wasm32")))]
use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast;

/// Logs all messages in a broadcaser on a console
///
/// Useful for debugging tests.
#[allow(dead_code)]
pub(crate) fn log_broadcast<T>(name: &str, mut receiver: broadcast::Receiver<T>)
where
    T: std::fmt::Debug + Clone + Send + 'static,
{
    let name = name.to_string();
    task::spawn(async move {
        loop {
            if let Ok(message) = receiver.recv().await {
                #[cfg(not(target_arch = "wasm32"))]
                println!("{}: {:#?}", &name, message);
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("{}: {:#?}", &name, message).into());
            } else {
                break;
            }
        }
    });
}

/// Timeout error
#[derive(Error, Debug, Clone)]
pub enum Timeout {
    #[error("timeout after {0:?}")]
    Elapsed(Duration),
    #[error("expected timeout of {0:?} didn't happen")]
    ExpectationFailed(Duration),
}

/// Times out in 100ms or whatever `TIMEOUT` env var is set to
#[cfg(not(any(feature = "wasm-executor", target_arch = "wasm32")))]
pub async fn timeout<F, T>(f: F) -> Result<T, Timeout>
where
    F: Future<Output = T>,
{
    use std::str::FromStr;
    let env = std::env::var("TIMEOUT").or_else(|_| Ok("100".to_owned()))?;
    let ms = u64::from_str(&env).or_else(|_| Ok(100))?;
    let timeout = Duration::from_millis(ms);
    tokio::time::timeout(timeout, f)
        .await
        .map_err(|_| Timeout::Elapsed(timeout))
}

/// Returns `Ok(())` if the future times out in 100ms (or whatever `TIMEOUT` env var is set to)
#[cfg(not(any(feature = "wasm-executor", target_arch = "wasm32")))]
pub async fn expects_timeout<F, T>(f: F) -> Result<(), Timeout>
where
    F: Future<Output = T>,
{
    use std::str::FromStr;
    let env = std::env::var("TIMEOUT").or_else(|_| Ok("100".to_owned()))?;
    let ms = u64::from_str(&env).or_else(|_| Ok(100))?;
    let timeout = Duration::from_millis(ms);

    match tokio::time::timeout(timeout, f).await {
        Ok(_) => Err(Timeout::ExpectationFailed(timeout)),
        Err(_) => Ok(()),
    }
}

#[cfg(any(feature = "wasm-executor", target_arch = "wasm32"))]
mod wasm {
    use super::Timeout as Error;
    use instant::Instant;
    use pin_project::pin_project;
    use std::future::Future;
    use std::marker::PhantomData;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;

    #[pin_project]
    pub(crate) struct Timeout<F, T>
    where
        F: Future<Output = T>,
    {
        #[pin]
        future: F,
        instant: Instant,
        duration: Duration,
        phantom_data: PhantomData<T>,
    }

    impl<F, T> Timeout<F, T>
    where
        F: Future<Output = T>,
    {
        fn new(duration: Duration, future: F) -> Self {
            let instant = Instant::now();
            Self {
                future,
                instant,
                duration,
                phantom_data: PhantomData,
            }
        }
    }

    impl<F, T> Future for Timeout<F, T>
    where
        F: Future<Output = T>,
    {
        type Output = Result<T, Error>;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.project();
            if let Poll::Ready(v) = this.future.poll(cx) {
                return Poll::Ready(Ok(v));
            }

            if this.instant.elapsed() > *this.duration {
                return Poll::Ready(Err(Error::Elapsed(*this.duration)));
            }

            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    /// Times out in 500ms
    pub async fn timeout<F, T>(future: F) -> Result<T, Error>
    where
        F: Future<Output = T>,
    {
        let duration = Duration::from_millis(500);
        Timeout::new(duration, future).await
    }

    /// Returns `Ok(())` if the future times out in 500ms
    pub async fn expects_timeout<F, T>(future: F) -> Result<(), Error>
    where
        F: Future<Output = T>,
    {
        let duration = Duration::from_millis(500);
        match Timeout::new(duration, future).await {
            Ok(_) => Err(Error::ExpectationFailed(duration)),
            Err(Error::Elapsed(_)) => Ok(()),
            Err(_) => unreachable!(),
        }
    }
}
#[cfg(any(feature = "wasm-executor", target_arch = "wasm32"))]
pub(crate) use wasm::*;
