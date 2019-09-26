use crate::spi::Subscriber;
use crate::spi::Subscription;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Once;

pub(crate) struct EmptySubscription;

impl Subscription for EmptySubscription {
  fn request(&self, _n: usize) {}
  fn cancel(&self) {}
}

pub(crate) struct BlockSubscriber<T, E> {
  tx: Sender<Result<Option<T>, E>>,
  once: Once,
}

impl<T, E> BlockSubscriber<T, E> {
  pub(crate) fn new() -> (BlockSubscriber<T, E>, Receiver<Result<Option<T>, E>>) {
    let (tx, rx) = channel();
    let sub = BlockSubscriber {
      tx,
      once: Once::new(),
    };
    (sub, rx)
  }
}

impl<T, E> Subscriber for BlockSubscriber<T, E> {
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    let tx = self.tx.clone();
    self.once.call_once(|| {
      tx.send(Ok(None)).unwrap();
    });
  }

  fn on_next(&self, t: T) {
    let tx = self.tx.clone();
    self.once.call_once(|| {
      tx.send(Ok(Some(t))).unwrap();
    });
  }

  fn on_error(&self, e: Self::Error) {
    let tx = self.tx.clone();
    self.once.call_once(|| {
      tx.clone().send(Err(e)).unwrap();
    });
  }
}
