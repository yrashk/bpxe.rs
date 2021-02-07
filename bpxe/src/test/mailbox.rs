//! # Mailbox testing harness
//!
use crate::test::*;
use async_trait::async_trait;
use bpxe_internal_macros as bpxe_im;
use tokio::sync::broadcast;

/// Message receiver
#[async_trait]
pub(crate) trait Receiver<M> {
    /// Message receiver error
    type Error;

    /// Asynchronously receive a message
    async fn recv(&mut self) -> Result<M, Self::Error>;

    /// Returns whether returned error signifies source closure
    fn is_closed(err: Self::Error) -> bool;
}

#[async_trait]
impl<M> Receiver<M> for broadcast::Receiver<M>
where
    M: Clone + Send,
{
    type Error = broadcast::error::RecvError;

    async fn recv(&mut self) -> Result<M, Self::Error> {
        (self as &mut broadcast::Receiver<M>).recv().await
    }

    fn is_closed(err: Self::Error) -> bool {
        match err {
            broadcast::error::RecvError::Closed => true,
            _ => false,
        }
    }
}

/// Message mailbox
pub(crate) struct Mailbox<M, R> {
    messages: Vec<M>,
    receiver: R,
}

impl<M, R> Mailbox<M, R> {
    pub fn new(receiver: R) -> Self {
        Self {
            messages: Default::default(),
            receiver,
        }
    }
}

impl<M, R> Mailbox<M, R>
where
    R: Receiver<M>,
{
    /// Returns if there was or will be a message that makes `predicate` function return true
    pub async fn receive<F>(&mut self, predicate: F) -> bool
    where
        F: Fn(&M) -> bool,
    {
        let mut received = false;
        self.messages = std::mem::replace(&mut self.messages, vec![])
            .into_iter()
            .filter(|message| {
                if predicate(&message) {
                    received = true;
                    false
                } else {
                    true
                }
            })
            .collect();
        if !received {
            loop {
                tokio::task::yield_now().await;
                match self.receiver.recv().await {
                    Ok(message) => {
                        if predicate(&message) {
                            received = true;
                            break;
                        } else {
                            self.messages.push(message);
                        }
                    }
                    Err(e) => {
                        if R::is_closed(e) {
                            break;
                        }
                    }
                }
            }
        }
        return received;
    }
}

#[bpxe_im::test]
async fn assert_receive() {
    let (sender, receiver) = broadcast::channel(16);
    let mut mailbox = Mailbox::new(receiver);
    sender.send(1u8).unwrap();
    sender.send(2u8).unwrap();
    assert!(mailbox.receive(|m| *m == 2u8).await);
    assert!(mailbox.receive(|m| *m == 1u8).await);
}

#[tokio::test]
async fn refute_receive() {
    let (sender, receiver) = broadcast::channel(16);
    let mut mailbox = Mailbox::new(receiver);
    sender.send(1u8).unwrap();
    sender.send(2u8).unwrap();
    assert!(expects_timeout(mailbox.receive(|m| *m == 3u8))
        .await
        .is_ok());
}
