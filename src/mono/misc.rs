use crate::spi::{Subscriber, Subscription, REQUEST_MAX};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Once;

type BlockSender<T, E> = Sender<Result<Option<T>, E>>;
type BlockReceiver<T, E> = Receiver<Result<Option<T>, E>>;

pub(crate) struct EmptySubscription;

impl Subscription for EmptySubscription {
  fn request(&self, _n: usize) {}
  fn cancel(&self) {}
}

pub(crate) struct BlockSubscriber<T, E> {
  tx: BlockSender<T, E>,
  once: Once,
}

impl<T, E> BlockSubscriber<T, E> {
  pub(crate) fn new() -> (BlockSubscriber<T, E>, BlockReceiver<T, E>) {
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

  fn on_subscribe(&self, subscription: impl Subscription) {
    subscription.request(REQUEST_MAX);
  }

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

  fn on_error(&self, e: E) {
    let tx = self.tx.clone();
    self.once.call_once(|| {
      tx.send(Err(e)).unwrap();
    });
  }
}
