use super::misc::EmptySubscription;
use super::spi::Mono;
use crate::spi::{Publisher, Subscriber, Subscription, REQUEST_MAX};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once};

pub struct MonoFlatMap<T1, T2, E, M1, M2, F>
where
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  M1: 'static + Send + Mono<T1, E>,
  M2: 'static + Send + Mono<T2, E>,
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
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  M1: 'static + Send + Mono<T1, E>,
  M2: 'static + Send + Mono<T2, E>,
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
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  M1: 'static + Send + Mono<T1, E>,
  M2: 'static + Send + Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M2,
{
}

impl<T1, T2, E, M1, M2, F> Publisher for MonoFlatMap<T1, T2, E, M1, M2, F>
where
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  M1: 'static + Send + Mono<T1, E>,
  M2: 'static + Send + Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M2,
{
  type Item = T2;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T2, Error = E> + 'static + Send) {
    let actual = Arc::new(Mutex::new(subscriber));
    let s = MainSubscriber::new(actual.clone(), self.mapper);
    actual
      .clone()
      .lock()
      .unwrap()
      .on_subscribe(EmptySubscription);
    self.source.subscribe(s);
  }
}

struct MainSubscriber<T1, T2, E, M, S, F>
where
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: 'static + Send + Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  actual: Arc<Mutex<S>>,
  mapper: F,
  once: Once,
  completed: Arc<AtomicBool>,
  _m: PhantomData<M>,
  _t1: PhantomData<T1>,
}

impl<T1, T2, E, M, S, F> MainSubscriber<T1, T2, E, M, S, F>
where
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: 'static + Send + Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  fn new(actual: Arc<Mutex<S>>, mapper: F) -> MainSubscriber<T1, T2, E, M, S, F> {
    MainSubscriber {
      actual,
      mapper,
      completed: Arc::new(AtomicBool::new(false)),
      once: Once::new(),
      _m: PhantomData,
      _t1: PhantomData,
    }
  }
}

impl<T1, T2, E, M, S, F> Subscriber for MainSubscriber<T1, T2, E, M, S, F>
where
  T1: 'static + Send,
  T2: 'static + Send,
  E: 'static + Send,
  S: 'static + Send + Subscriber<Item = T2, Error = E>,
  M: 'static + Send + Mono<T2, E>,
  F: 'static + Send + Fn(T1) -> M,
{
  type Item = T1;
  type Error = E;

  fn on_subscribe(&self, subscription: impl Subscription) {
    subscription.request(REQUEST_MAX);
  }

  fn on_complete(&self) {
    let actual = self.actual.clone();
    let completed = self.completed.clone();
    self.once.call_once(|| {
      if completed.load(Ordering::SeqCst) {
        let v = actual.lock().unwrap();
        v.on_complete();
      }
    });
  }
  fn on_next(&self, t: T1) {
    let m2 = (self.mapper)(t);
    let inner = InnerSubscriber::new(self.actual.clone(), self.completed.clone());
    m2.subscribe(inner)
  }
  fn on_error(&self, e: E) {
    let actual = self.actual.clone();
    self.once.call_once(move || {
      let s = actual.lock().unwrap();
      s.on_error(e);
    });
  }
}

struct InnerSubscriber<T, E, S>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
{
  actual: Arc<Mutex<S>>,
  completed: Arc<AtomicBool>,
  once: Once,
}

impl<T, E, S> InnerSubscriber<T, E, S>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
{
  fn new(actual: Arc<Mutex<S>>, completed: Arc<AtomicBool>) -> InnerSubscriber<T, E, S> {
    InnerSubscriber {
      actual,
      completed,
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

  fn on_subscribe(&self, subscription: impl Subscription) {
    subscription.request(REQUEST_MAX);
  }

  fn on_complete(&self) {
    let actual = self.actual.clone();
    let completed = self.completed.clone();
    self.once.call_once(move || {
      let s = actual.lock().unwrap();
      if completed.compare_and_swap(false, true, Ordering::SeqCst) {
        s.on_complete();
      }
    });
  }
  fn on_next(&self, t: T) {
    self.actual.clone().lock().unwrap().on_next(t);
  }

  fn on_error(&self, e: E) {
    let actual = self.actual.clone();
    self.once.call_once(move || {
      let s = actual.lock().unwrap();
      s.on_error(e);
    });
  }
}
