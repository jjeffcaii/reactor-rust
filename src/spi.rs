use std::marker::PhantomData;

pub const REQUEST_MAX: usize = 0x7fff_ffff;

pub trait Publisher: Sized {
  type Item;
  type Error;
  fn subscribe(
    self,
    subscriber: impl Subscriber<Item = Self::Item, Error = Self::Error> + 'static + Send,
  );
}

pub trait Subscription: Sized {
  fn request(&self, n: usize);
  fn cancel(&self);
}

pub trait Subscriber {
  type Item;
  type Error;
  fn on_complete(&self);
  fn on_next(&self, t: Self::Item)
  where
    Self: Sized;

  fn on_subscribe(&self, subscription: impl Subscription)
  where
    Self: Sized;

  fn on_error(&self, e: Self::Error)
  where
    Self: Sized;
}

pub struct Subscribers;

impl Subscribers {
  pub fn next<T, E, F>(action: F) -> impl Subscriber<Item = T, Error = E>
  where
    F: Fn(T),
  {
    NextSubscriber::new(action)
  }

  pub fn noop<T, E>() -> impl Subscriber<Item = T, Error = E> {
    NoopSubscriber::new()
  }
}

pub(crate) struct NextSubscriber<T, E, F>
where
  F: Fn(T),
{
  f: F,
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E, F> NextSubscriber<T, E, F>
where
  F: Fn(T),
{
  pub(crate) fn new(f: F) -> NextSubscriber<T, E, F> {
    NextSubscriber {
      f,
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E, F> Subscriber for NextSubscriber<T, E, F>
where
  F: Fn(T),
{
  type Item = T;
  type Error = E;

  fn on_subscribe(&self, subscription: impl Subscription) {
    subscription.request(REQUEST_MAX);
  }

  fn on_complete(&self) {}
  fn on_next(&self, t: T) {
    (self.f)(t)
  }
  fn on_error(&self, _e: E) {}
}

pub(crate) struct NoopSubscriber<T, E> {
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E> NoopSubscriber<T, E> {
  pub(crate) fn new() -> NoopSubscriber<T, E> {
    NoopSubscriber {
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E> Subscriber for NoopSubscriber<T, E> {
  type Item = T;
  type Error = E;
  fn on_subscribe(&self, subscription: impl Subscription) {
    subscription.request(REQUEST_MAX);
  }
  fn on_complete(&self) {}
  fn on_next(&self, _t: T) {}
  fn on_error(&self, _e: E) {}
}
