use super::emitter::Emitter;
use super::misc::EmptySubscription;
use crate::spi::{Publisher, Subscriber, Subscription};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Processor<T, E> {
  tx: Sender<Result<T, E>>,
  rx: Receiver<Result<T, E>>,
}

impl<T, E> Processor<T, E> {
  pub fn new() -> Processor<T, E> {
    let (tx, rx) = channel();
    Processor { tx, rx }
  }

  pub fn emitter(&self) -> Emitter<T, E> {
    Emitter::new(self.tx.clone())
  }
}

impl<T, E> Publisher for Processor<T, E> {
  type Item = T;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E> + 'static + Send) {
    let s = InnerSubscriber::new(subscriber, self.rx);
    s.on_subscribe(EmptySubscription);
  }
}

struct InnerSubscription<T, E> {
  emitter: Emitter<T, E>,
}

impl<T, E> Subscription for InnerSubscription<T, E> {
  fn request(&self, n: usize) {}
  fn cancel(&self) {}
}

struct InnerSubscriber<T, E, S>
where
  S: Subscriber<Item = T, Error = E>,
{
  actual: S,
  rx: Receiver<Result<T, E>>,
}

impl<T, E, S> InnerSubscriber<T, E, S>
where
  S: Subscriber<Item = T, Error = E>,
{
  fn new(actual: S, rx: Receiver<Result<T, E>>) -> InnerSubscriber<T, E, S> {
    InnerSubscriber { actual, rx }
  }
}

impl<T, E, S> Subscriber for InnerSubscriber<T, E, S>
where
  S: Subscriber<Item = T, Error = E>,
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    self.actual.on_complete();
  }
  fn on_next(&self, t: T) {
    self.actual.on_next(t);
  }

  fn on_subscribe(&self, subscription: impl Subscription) {
    self.actual.on_subscribe(subscription);
    match self.rx.recv().unwrap() {
      Ok(t) => {
        self.on_next(t);
        self.on_complete();
      }
      Err(e) => {
        self.on_error(e);
      }
    }
  }

  fn on_error(&self, e: E) {
    self.actual.on_error(e);
  }
}
