use super::Mono;
use crate::spi::Subscriber;
use std::rc::Rc;

pub struct DoOnError<T, E, M, F>
where
  T: 'static,
  E: 'static,
  M: Mono<Item = T, Error = E> + Sized,
  F: 'static + Send + Fn(&E),
{
  source: M,
  f: F,
}

impl<T, E, M, F> DoOnError<T, E, M, F>
where
  T: 'static,
  E: 'static,
  M: Mono<Item = T, Error = E> + Sized,
  F: 'static + Send + Fn(&E),
{
  pub(crate) fn new(source: M, f: F) -> DoOnError<T, E, M, F> {
    DoOnError { source, f }
  }
}

impl<T, E, M, F> Mono for DoOnError<T, E, M, F>
where
  M: Mono<Item = T, Error = E> + Sized,
  F: 'static + Send + Fn(&E),
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    Self: Sized,
    S: 'static + Send + Subscriber<Item = T, Error = E>,
  {
    let sub = DoOnErrorSubscriber::new(subscriber, self.f);
    self.source.subscribe(sub);
  }
}

struct DoOnErrorSubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&E),
{
  actual: S,
  action: F,
}

impl<T, E, S, F> DoOnErrorSubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&E),
{
  fn new(actual: S, action: F) -> DoOnErrorSubscriber<T, E, S, F> {
    DoOnErrorSubscriber { actual, action }
  }
}

impl<T, E, S, F> Subscriber for DoOnErrorSubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&E),
{
  type Item = T;
  type Error = E;
  fn on_complete(&self) {
    self.actual.on_complete();
  }
  fn on_next(&self, t: Self::Item) {
    self.actual.on_next(t);
  }
  fn on_error(&self, e: Self::Error) {
    (self.action)(&e);
    self.actual.on_error(e);
  }
}
