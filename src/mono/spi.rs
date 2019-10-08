extern crate futures;

use super::misc::BlockSubscriber;
use super::{
  Foreach, MonoDoFinally, MonoDoOnComplete, MonoDoOnError, MonoFilter, MonoFlatMap, MonoScheduleOn,
  MonoTransform,
};
use crate::schedulers::Scheduler;
use crate::spi::Publisher;

pub trait Mono<T, E>: Publisher<Item = T, Error = E> {
  fn block(self) -> Result<Option<Self::Item>, Self::Error>
  where
    Self::Item: 'static + Send,
    Self::Error: 'static + Send,
  {
    let (sub, rx) = BlockSubscriber::new();
    self.subscribe(sub);
    rx.recv().unwrap()
  }

  fn do_on_error<F>(self, f: F) -> MonoDoOnError<Self::Item, Self::Error, Self, F>
  where
    F: 'static + Send + Fn(&Self::Error),
  {
    MonoDoOnError::new(self, f)
  }

  fn do_on_success<F>(self, f: F) -> Foreach<Self, Self::Item, F, Self::Error>
  where
    F: 'static + Send + Fn(&Self::Item),
  {
    Foreach::new(self, f)
  }

  fn do_on_complete<F>(self, f: F) -> MonoDoOnComplete<Self::Item, Self::Error, Self, F>
  where
    F: 'static + Send + Fn(),
  {
    MonoDoOnComplete::new(self, f)
  }

  fn map<A, F>(self, transform: F) -> MonoTransform<Self, Self::Item, A, F, Self::Error>
  where
    F: 'static + Send + Fn(Self::Item) -> A,
  {
    MonoTransform::new(self, transform)
  }

  fn flatmap<A, M, F>(self, mapper: F) -> MonoFlatMap<Self::Item, A, Self::Error, Self, M, F>
  where
    Self: 'static + Send,
    Self::Item: 'static + Send,
    Self::Error: 'static + Send,
    A: 'static + Send,
    M: 'static + Send + Mono<A, Self::Error>,
    F: 'static + Send + Fn(Self::Item) -> M,
  {
    MonoFlatMap::new(self, mapper)
  }

  fn do_finally<F>(self, action: F) -> MonoDoFinally<Self::Item, Self::Error, Self, F>
  where
    F: 'static + Send + Fn(),
  {
    MonoDoFinally::new(self, action)
  }

  fn filter<F>(self, predicate: F) -> MonoFilter<Self, Self::Item, F, Self::Error>
  where
    F: 'static + Send + Fn(&Self::Item) -> bool,
  {
    MonoFilter::new(self, predicate)
  }

  fn subscribe_on<C>(self, scheduler: C) -> MonoScheduleOn<Self::Item, Self::Error, Self, C>
  where
    Self: 'static + Send,
    C: Scheduler<Item = Self::Item, Error = Self::Error>,
  {
    MonoScheduleOn::new(self, scheduler)
  }
}
