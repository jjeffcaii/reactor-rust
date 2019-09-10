use std::marker::PhantomData;

pub const REQUEST_MAX: usize = 0x7fff_ffff;

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
    Self: Sized,
  {
    subscription.request(REQUEST_MAX)
  }

  fn on_error(&self, e: Self::Error)
  where
    Self: Sized;
}

pub struct CoreSubscriber<T, E, A, B, C>
where
  A: Fn(),
  B: Fn(T),
  C: Fn(E),
{
  fn_complete: A,
  fn_next: B,
  fn_error: C,
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E, A, B, C> CoreSubscriber<T, E, A, B, C>
where
  A: Fn(),
  B: Fn(T),
  C: Fn(E),
{
  pub fn new(fn_complete: A, fn_next: B, fn_error: C) -> CoreSubscriber<T, E, A, B, C> {
    CoreSubscriber {
      fn_complete,
      fn_next,
      fn_error,
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E, A, B, C> Subscriber for CoreSubscriber<T, E, A, B, C>
where
  A: Fn(),
  B: Fn(T),
  C: Fn(E),
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    (self.fn_complete)()
  }
  fn on_next(&self, t: T) {
    (self.fn_next)(t)
  }
  fn on_error(&self, e: E) {
    (self.fn_error)(e)
  }
}

pub struct Subscribers;

impl Subscribers {
  pub fn noop<T, E>() -> impl Subscriber<Item = T, Error = E> {
    NoopSubscriber::new()
  }
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

  fn on_complete(&self) {}
  fn on_next(&self, _t: T) {}
  fn on_error(&self, _e: E) {}
}
