use super::Mono;
use crate::spi::Subscriber;
use std::marker::PhantomData;
use std::thread;

const THREAD_NAME: &str = "rx";

pub struct MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<Item = T, Error = E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
  source: M,
  scheduler: C,
}

impl<T, E, M, C> MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<Item = T, Error = E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
  pub fn new(source: M, scheduler: C) -> MonoScheduleOn<T, E, M, C> {
    MonoScheduleOn { source, scheduler }
  }
}

impl<T, E, M, C> Mono for MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<Item = T, Error = E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: 'static + Send + Subscriber<Item = T, Error = E>,
  {
    self.scheduler.schedule(self.source, subscriber);
  }
}

pub trait Scheduler {
  type Item;
  type Error;
  fn schedule<P, S>(&self, publisher: P, subscriber: S)
  where
    Self: Sized,
    P: 'static + Send + Sized + Mono<Item = Self::Item, Error = Self::Error>,
    S: 'static + Send + Sized + Subscriber<Item = Self::Item, Error = Self::Error>;
}

pub struct Schedulers;

impl Schedulers {
  pub fn immediate<T, E>() -> ImmediateScheduler<T, E>
  where
    T: 'static,
    E: 'static,
  {
    ImmediateScheduler::new()
  }
  pub fn new_thread<T, E>() -> NewThreadScheduler<T, E>
  where
    T: 'static,
    E: 'static,
  {
    NewThreadScheduler::new()
  }
}

pub struct ImmediateScheduler<T, E> {
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
    P: 'static + Send + Sized + Mono<Item = T, Error = E>,
    S: 'static + Send + Sized + Subscriber<Item = T, Error = E>,
  {
    publisher.subscribe(subscriber);
  }
}

pub struct NewThreadScheduler<T, E> {
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
    P: 'static + Send + Sized + Mono<Item = T, Error = E>,
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
