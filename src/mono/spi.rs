use crate::mono::{DoOnError, Foreach, MonoFilter, MonoScheduleOn, MonoTransform, Scheduler};
use crate::spi::{CoreSubscriber, Subscriber};
use std::sync::mpsc::channel;

pub trait Mono {
  type Item;
  type Error;

  fn subscribe<S>(self, subscriber: S)
  where
    Self: Sized,
    S: 'static + Send + Subscriber<Item = Self::Item, Error = Self::Error>;

  fn block(self) -> Result<Self::Item, Self::Error>
  where
    Self::Item: 'static + Send,
    Self::Error: 'static + Send,
    Self: Sized,
  {
    let (tx, rx) = channel();
    let tx2 = tx.clone();
    self.subscribe(CoreSubscriber::new(
      || {},
      move |it| {
        tx.send(Ok(it)).unwrap();
      },
      move |e| {
        tx2.send(Err(e)).unwrap();
      },
    ));
    rx.recv().unwrap()
  }

  fn do_on_error<F>(self, f: F) -> DoOnError<Self::Item, Self::Error, Self, F>
  where
    F: 'static + Send + Fn(&Self::Error),
    Self: Sized,
  {
    DoOnError::new(self, f)
  }

  fn do_on_success<F>(self, f: F) -> Foreach<Self, Self::Item, F, Self::Error>
  where
    F: 'static + Send + Fn(&Self::Item),
    Self: Sized,
  {
    Foreach::new(self, f)
  }

  fn map<T, F>(self, transform: F) -> MonoTransform<Self, Self::Item, T, F, Self::Error>
  where
    F: 'static + Send + Fn(Self::Item) -> T,
    Self: Sized,
  {
    MonoTransform::new(self, transform)
  }

  fn filter<F>(self, predicate: F) -> MonoFilter<Self, Self::Item, F, Self::Error>
  where
    Self: Sized,
    F: 'static + Send + Fn(&Self::Item) -> bool,
  {
    MonoFilter::new(self, predicate)
  }

  fn subscribe_on<C>(self, scheduler: C) -> MonoScheduleOn<Self::Item, Self::Error, Self, C>
  where
    Self: 'static + Send + Sized,
    C: Scheduler<Item = Self::Item, Error = Self::Error>,
  {
    MonoScheduleOn::new(self, scheduler)
  }
}
