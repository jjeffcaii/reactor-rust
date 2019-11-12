use crate::spi::{Publisher, Subscriber};
use std::marker::PhantomData;
use std::thread;

const THREAD_NAME: &str = "rx";

pub trait Scheduler {
  type Item;
  type Error;
  fn schedule<P, S>(&self, publisher: P, subscriber: S)
  where
    Self: Sized,
    P: 'static + Send + Sized + Publisher<Item = Self::Item, Error = Self::Error>,
    S: 'static + Send + Sized + Subscriber<Item = Self::Item, Error = Self::Error>;
}

pub fn immediate<T, E>() -> impl Scheduler<Item = T, Error = E> {
  ImmediateScheduler::new()
}
pub fn new_thread<T, E>() -> impl Scheduler<Item = T, Error = E>
where
  T: 'static,
  E: 'static,
{
  NewThreadScheduler::new()
}

struct ImmediateScheduler<T, E> {
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E> ImmediateScheduler<T, E> {
  pub(crate) fn new() -> ImmediateScheduler<T, E> {
    ImmediateScheduler {
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E> Scheduler for ImmediateScheduler<T, E> {
  type Item = T;
  type Error = E;

  fn schedule<P, S>(&self, publisher: P, subscriber: S)
  where
    P: 'static + Send + Sized + Publisher<Item = Self::Item, Error = Self::Error>,
    S: 'static + Send + Sized + Subscriber<Item = T, Error = E>,
  {
    publisher.subscribe(subscriber);
  }
}

struct NewThreadScheduler<T, E> {
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E> NewThreadScheduler<T, E> {
  pub(crate) fn new() -> NewThreadScheduler<T, E> {
    NewThreadScheduler {
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E> Scheduler for NewThreadScheduler<T, E> {
  type Item = T;
  type Error = E;

  fn schedule<P, S>(&self, publisher: P, subscriber: S)
  where
    P: 'static + Send + Sized + Publisher<Item = Self::Item, Error = Self::Error>,
    S: 'static + Send + Sized + Subscriber<Item = T, Error = E>,
  {
    thread::Builder::new()
      .name(String::from(THREAD_NAME))
      .spawn(move || {
        publisher.subscribe(subscriber);
      })
      .unwrap();
  }
}
