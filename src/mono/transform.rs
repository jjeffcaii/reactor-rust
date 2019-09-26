use super::spi::Mono;
use crate::spi::{Publisher, Subscriber};
use std::marker::PhantomData;

pub struct MonoTransform<M, T1, T2, F, E>
where
  T1: 'static,
  T2: 'static,
  E: 'static,
  M: Mono<T1, E> + Sized,
  F: 'static + Send + Fn(T1) -> T2,
{
  parent: M,
  transformer: F,
  _t1: PhantomData<T1>,
  _t2: PhantomData<T2>,
  _e: PhantomData<E>,
}

impl<M, T1, T2, F, E> MonoTransform<M, T1, T2, F, E>
where
  M: Mono<T1, E> + Sized,
  F: 'static + Send + Fn(T1) -> T2,
{
  pub(crate) fn new(m: M, f: F) -> MonoTransform<M, T1, T2, F, E> {
    MonoTransform {
      parent: m,
      transformer: f,
      _t1: PhantomData,
      _t2: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<M, T1, T2, F, E> Mono<T2, E> for MonoTransform<M, T1, T2, F, E>
where
  T1: Send,
  T2: Send,
  M: Mono<T1, E> + Sized,
  F: 'static + Send + Fn(T1) -> T2,
{
}

impl<M, T1, T2, F, E> Publisher for MonoTransform<M, T1, T2, F, E>
where
  T1: Send,
  T2: Send,
  M: Mono<T1, E> + Sized,
  F: 'static + Send + Fn(T1) -> T2,
{
  type Item = T2;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: 'static + Send + Subscriber<Item = T2, Error = E>,
  {
    self
      .parent
      .subscribe(TransformSubscriber::new(subscriber, self.transformer));
  }
}

struct TransformSubscriber<T1, T2, F, S, E>
where
  F: 'static + Send + Fn(T1) -> T2,
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
{
  actual: S,
  transformer: F,
  _marker: PhantomData<T1>,
}

impl<T1, T2, F, S, E> TransformSubscriber<T1, T2, F, S, E>
where
  F: 'static + Send + Fn(T1) -> T2,
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
{
  fn new(actual: S, transformer: F) -> TransformSubscriber<T1, T2, F, S, E> {
    TransformSubscriber {
      actual,
      transformer,
      _marker: PhantomData,
    }
  }
}

impl<T1, T2, F, S, E> Subscriber for TransformSubscriber<T1, T2, F, S, E>
where
  F: 'static + Send + Fn(T1) -> T2,
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
{
  type Item = T1;
  type Error = E;

  fn on_complete(&self) {
    self.actual.on_complete();
  }
  fn on_next(&self, t: T1) {
    let t2 = (self.transformer)(t);
    self.actual.on_next(t2);
  }
  fn on_error(&self, e: E) {
    self.actual.on_error(e);
  }
}
