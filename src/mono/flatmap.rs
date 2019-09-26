use super::spi::Mono;
use crate::spi::{Publisher, Subscriber, Subscription};
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Once;

pub struct MonoFlatMap<T1, T2, E, M1, M2, F>
where
  M1: Mono<T1, E>,
  M2: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M2,
{
  source: M1,
  mapper: F,
  _t1: PhantomData<T1>,
  _t2: PhantomData<T2>,
  _e: PhantomData<E>,
}

impl<T1, T2, E, M1, M2, F> MonoFlatMap<T1, T2, E, M1, M2, F>
where
  M1: Mono<T1, E>,
  M2: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M2,
{
  pub(crate) fn new(source: M1, mapper: F) -> MonoFlatMap<T1, T2, E, M1, M2, F> {
    MonoFlatMap {
      source,
      mapper,
      _t1: PhantomData,
      _t2: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T1, T2, E, M1, M2, F> Mono<T2, E> for MonoFlatMap<T1, T2, E, M1, M2, F>
where
  M1: Mono<T1, E>,
  M2: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M2,
{
}

impl<T1, T2, E, M1, M2, F> Publisher for MonoFlatMap<T1, T2, E, M1, M2, F>
where
  M1: Mono<T1, E>,
  M2: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M2,
{
  type Item = T2;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: 'static + Send + Subscriber<Item = T2, Error = E>,
  {
    let actual = Arc::new(subscriber);
    let s = MainSubscriber::new(actual.clone(), self.mapper);
    actual.on_subscribe(s);
    // self.source.subscribe(s);
  }
}

struct MainSubscriber<T1, T2, E, M, S, F>
where
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  actual: Arc<S>,
  mapper: F,
  once: Once,
  _m: PhantomData<M>,
  _t1: PhantomData<T1>,
}

impl<T1, T2, E, M, S, F> MainSubscriber<T1, T2, E, M, S, F>
where
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  fn new(actual: Arc<S>, mapper: F) -> MainSubscriber<T1, T2, E, M, S, F> {
    MainSubscriber {
      actual,
      mapper,
      once: Once::new(),
      _m: PhantomData,
      _t1: PhantomData,
    }
  }
}

impl<T1, T2, E, M, S, F> Subscriber for MainSubscriber<T1, T2, E, M, S, F>
where
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  type Item = T1;
  type Error = E;

  fn on_complete(&self) {
    self.once.call_once(|| self.actual.on_complete());
  }
  fn on_next(&self, t: T1) {
    let m2 = (self.mapper)(t);
    // TODO: create inner subscriber
    let inner = InnerSubscriber::new(self.actual.clone());
    // m2.subscribe(inner)
  }
  fn on_error(&self, e: E) {
    self.once.call_once(move || self.actual.on_error(e));
  }
}

impl<T1, T2, E, M, S, F> Subscription for MainSubscriber<T1, T2, E, M, S, F>
where
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  fn request(&self, n: usize) {}

  fn cancel(&self) {}
}

struct InnerSubscriber<T, E, S>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
{
  actual: Arc<S>,
  once: Once,
}

impl<T, E, S> InnerSubscriber<T, E, S>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
{
  fn new(actual: Arc<S>) -> InnerSubscriber<T, E, S> {
    InnerSubscriber {
      actual,
      once: Once::new(),
    }
  }
}

impl<T, E, S> Subscriber for InnerSubscriber<T, E, S>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    self.once.call_once(|| self.actual.on_complete());
  }
  fn on_next(&self, t: T) {
    self.actual.on_next(t);
  }

  fn on_error(&self, e: E) {
    self.once.call_once(|| self.actual.on_error(e));
  }
}
