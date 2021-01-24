//! # Testing Helpers
//!
mod mailbox;
pub(crate) use mailbox::Mailbox;

use std::future::Future;
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio::task;

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
                println!("{}: {:#?}", &name, message);
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
}

/// Times out in 100ms or whatever `TIMEOUT` env var is set to
pub async fn timeout<F, T>(future: F) -> Result<T, Timeout>
where
    F: Future<Output = T>,
{
    let env = std::env::var("TIMEOUT").or_else(|_| Ok("100".to_owned()))?;
    let ms = u64::from_str(&env).or_else(|_| Ok(100))?;
    let timeout = Duration::from_millis(ms);
    tokio::time::timeout(timeout, future)
        .await
        .map_err(|_| Timeout::Elapsed(timeout))
}
