use super::spi::Mono;
use crate::spi::{Publisher, Subscriber, Subscription};
use std::marker::PhantomData;

pub struct MonoTransformError<M, T, F, E1, E2>
where
  T: 'static,
  E1: 'static,
  E2: 'static,
  M: Mono<T, E1> + Sized,
  F: 'static + Send + Fn(E1) -> E2,
{
  source: M,
  transformer: F,
  _t: PhantomData<T>,
  _e1: PhantomData<E1>,
  _e2: PhantomData<E2>,
}

impl<M, T, F, E1, E2> MonoTransformError<M, T, F, E1, E2>
where
  M: Mono<T, E1> + Sized,
  F: 'static + Send + Fn(E1) -> E2,
{
  pub(crate) fn new(m: M, f: F) -> MonoTransformError<M, T, F, E1, E2> {
    MonoTransformError {
      source: m,
      transformer: f,
      _t: PhantomData,
      _e1: PhantomData,
      _e2: PhantomData,
    }
  }
}

impl<M, T, F, E1, E2> Mono<T, E2> for MonoTransformError<M, T, F, E1, E2>
where
  E1: Send,
  E2: Send,
  M: Mono<T, E1> + Sized,
  F: 'static + Send + Fn(E1) -> E2,
{
}

impl<M, T, F, E1, E2> Publisher for MonoTransformError<M, T, F, E1, E2>
where
  E1: Send,
  E2: Send,
  M: Mono<T, E1> + Sized,
  F: 'static + Send + Fn(E1) -> E2,
{
  type Item = T;
  type Error = E2;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E2> + 'static + Send) {
    self
      .source
      .subscribe(InnerSubscriber::new(subscriber, self.transformer));
  }
}

struct InnerSubscriber<T, F, S, E1, E2>
where
  F: 'static + Send + Fn(E1) -> E2,
  S: 'static + Send + Subscriber<Item = T, Error = E2>,
{
  actual: S,
  transformer: F,
  _e1: PhantomData<E1>,
}

impl<T, F, S, E1, E2> InnerSubscriber<T, F, S, E1, E2>
where
  F: 'static + Send + Fn(E1) -> E2,
  S: 'static + Send + Subscriber<Item = T, Error = E2>,
{
  fn new(actual: S, transformer: F) -> InnerSubscriber<T, F, S, E1, E2> {
    InnerSubscriber {
      actual,
      transformer,
      _e1: PhantomData,
    }
  }
}

impl<T, F, S, E1, E2> Subscriber for InnerSubscriber<T, F, S, E1, E2>
where
  F: 'static + Send + Fn(E1) -> E2,
  S: 'static + Send + Subscriber<Item = T, Error = E2>,
{
  type Item = T;
  type Error = E1;

  fn on_complete(&self) {
    self.actual.on_complete();
  }
  fn on_next(&self, t: T) {
    self.actual.on_next(t);
  }

  fn on_subscribe(&self, subscription: impl Subscription) {
    self.actual.on_subscribe(subscription);
  }

  fn on_error(&self, e: E1) {
    let e2 = (self.transformer)(e);
    self.actual.on_error(e2);
  }
}
