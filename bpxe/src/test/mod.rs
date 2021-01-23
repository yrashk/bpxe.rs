//! # Testing Helpers
//!
mod mailbox;
pub(crate) use mailbox::Mailbox;

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
