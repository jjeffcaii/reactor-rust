use super::spi::Mono;
use crate::spi::{Subscriber, Subscription, Publisher};
use std::sync::Once;
use std::marker::PhantomData;

pub struct MonoDoFinally<T, E, M, F>
where
  T: 'static,
  E: 'static,
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
  source: M,
  action: F,
  _t: PhantomData<T>,
  _e: PhantomData<E>
}

impl<T, E, M, F> MonoDoFinally<T, E, M, F>
where
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
  pub(crate) fn new(source: M, action: F) -> MonoDoFinally<T, E, M, F> {
    MonoDoFinally { source, action,_t: PhantomData,_e: PhantomData, }
  }
}

impl<T, E, M, F> Mono<T,E> for MonoDoFinally<T, E, M, F>
where
  M: Mono<T, E>,
  F: 'static + Send + Fn(),
{
}

impl<T, E, M, F> Publisher for MonoDoFinally<T, E, M, F>
where
  M: Mono<T,E>,
  F: 'static + Send + Fn(),
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: 'static + Send + Subscriber<Item = T, Error = E>,
  {
    let sub = DoFinallySubscriber::new(subscriber, self.action);
    self.source.subscribe(sub);
  }
}

struct DoFinallySubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(),
{
  actual: S,
  action: F,
  once: Once,
}

impl<T, E, S, F> DoFinallySubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(),
{
  fn new(actual: S, action: F) -> DoFinallySubscriber<T, E, S, F> {
    DoFinallySubscriber {
      actual,
      action,
      once: Once::new(),
    }
  }

  fn finally(&self) {
    self.once.call_once(|| (self.action)());
  }
}

impl<T, E, S, F> Subscriber for DoFinallySubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(),
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    self.actual.on_complete();
    self.finally();
  }

  fn on_next(&self, t: T) {
    self.actual.on_next(t);
  }

  fn on_subscribe(&self, subscription: impl Subscription) {
    self.actual.on_subscribe(subscription);
  }

  fn on_error(&self, e: E) {
    self.actual.on_error(e);
    self.finally();
  }
}
