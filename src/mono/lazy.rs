use super::emitter::Emitter;
use super::spi::Mono;
use crate::spi::{Publisher, Subscriber, Subscription};
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver};

pub struct MonoLazy<T, E, F>
where
  F: Fn(&Emitter<T, E>),
{
  f: F,
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E, F> MonoLazy<T, E, F>
where
  F: Fn(&Emitter<T, E>),
{
  pub(crate) fn new(f: F) -> MonoLazy<T, E, F> {
    MonoLazy {
      f,
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E, F> Mono<T, E> for MonoLazy<T, E, F> where F: Fn(&Emitter<T, E>) {}

impl<T, E, F> Publisher for MonoLazy<T, E, F>
where
  F: Fn(&Emitter<T, E>),
{
  type Item = T;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E> + 'static + Send) {
    let (tx, rx) = channel();
    let s = InnerSubscriber::new(subscriber, rx);
    let su = InnerSubscription::new(Emitter::new(tx), self.f);
    s.on_subscribe(su);
  }
}

struct InnerSubscription<T, E, F>
where
  F: Fn(&Emitter<T, E>),
{
  emitter: Emitter<T, E>,
  f: F,
}

impl<T, E, F> InnerSubscription<T, E, F>
where
  F: Fn(&Emitter<T, E>),
{
  fn new(emitter: Emitter<T, E>, f: F) -> InnerSubscription<T, E, F> {
    InnerSubscription { emitter, f }
  }
}

impl<T, E, F> Subscription for InnerSubscription<T, E, F>
where
  F: Fn(&Emitter<T, E>),
{
  fn request(&self, _n: usize) {
    (self.f)(&self.emitter);
  }
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
