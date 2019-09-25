use crate::mono::Mono;
use crate::spi::Subscriber;

pub struct Foreach<Z, T, F, E>
where
  T: 'static,
  E: 'static,
  Z: Mono<Item = T, Error = E> + Sized,
  F: 'static + Send + Fn(&T),
{
  zero: Z,
  f: F,
}

impl<Z, T, F, E> Foreach<Z, T, F, E>
where
  Z: Mono<Item = T, Error = E> + Sized,
  F: 'static + Send + Fn(&T),
{
  pub(crate) fn new(zero: Z, f: F) -> Foreach<Z, T, F, E> {
    Foreach { zero, f }
  }
}

impl<Z, T, F, E> Mono for Foreach<Z, T, F, E>
where
  Z: Mono<Item = T, Error = E> + Sized,
  F: 'static + Send + Fn(&T),
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: 'static + Send + Subscriber<Item = T, Error = E>,
  {
    let sub = ForeachSubscriber::new(subscriber, self.f);
    self.zero.subscribe(sub);
  }
}

struct ForeachSubscriber<T, S, F, E>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&T),
{
  actual: S,
  action: F,
}

impl<T, S, F, E> ForeachSubscriber<T, S, F, E>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&T),
{
  fn new(actual: S, action: F) -> ForeachSubscriber<T, S, F, E> {
    ForeachSubscriber { actual, action }
  }
}

impl<T, S, F, E> Subscriber for ForeachSubscriber<T, S, F, E>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&T),
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    self.actual.on_complete()
  }

  fn on_next(&self, t: T) {
    (self.action)(&t);
    self.actual.on_next(t);
  }
  fn on_error(&self, e: E) {
    self.actual.on_error(e)
  }
}
