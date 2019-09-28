use super::spi::Mono;
use crate::spi::{Publisher, Subscriber, Subscription};
use std::marker::PhantomData;

pub struct MonoDoOnComplete<T, E, M, F>
where
  T: 'static,
  E: 'static,
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
  source: M,
  f: F,
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E, M, F> MonoDoOnComplete<T, E, M, F>
where
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
  pub(crate) fn new(source: M, f: F) -> MonoDoOnComplete<T, E, M, F> {
    MonoDoOnComplete {
      source,
      f,
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E, M, F> Mono<T, E> for MonoDoOnComplete<T, E, M, F>
where
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
}

impl<T, E, M, F> Publisher for MonoDoOnComplete<T, E, M, F>
where
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
  type Item = T;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E> + 'static + Send) {
    let s = CompleteSubscriber::new(subscriber, self.f);
    self.source.subscribe(s);
  }
}

struct CompleteSubscriber<T, E, S, F>
where
  S: Subscriber<Item = T, Error = E> + 'static + Send,
  F: 'static + Send + Fn(),
{
  actual: S,
  f: F,
}

impl<T, E, S, F> CompleteSubscriber<T, E, S, F>
where
  S: Subscriber<Item = T, Error = E> + 'static + Send,
  F: 'static + Send + Fn(),
{
  fn new(actual: S, f: F) -> CompleteSubscriber<T, E, S, F> {
    CompleteSubscriber { actual, f }
  }
}

impl<T, E, S, F> Subscriber for CompleteSubscriber<T, E, S, F>
where
  S: Subscriber<Item = T, Error = E> + 'static + Send,
  F: 'static + Send + Fn(),
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    (self.f)();
    self.actual.on_complete();
  }
  fn on_next(&self, t: T) {
    self.actual.on_next(t);
  }

  fn on_subscribe(&self, subscription: impl Subscription) {
    self.actual.on_subscribe(subscription);
  }

  fn on_error(&self, e: E) {
    self.actual.on_error(e);
  }
}
