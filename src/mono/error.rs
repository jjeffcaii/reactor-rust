use super::misc::EmptySubscription;
use super::spi::Mono;
use crate::spi::{Publisher, Subscriber};
use std::marker::PhantomData;

pub struct MonoError<T, E> {
  e: E,
  _t: PhantomData<T>,
}

impl<T, E> MonoError<T, E> {
  pub(crate) fn new(e: E) -> MonoError<T, E> {
    MonoError { e, _t: PhantomData }
  }
}

impl<T, E> Mono<T, E> for MonoError<T, E> {}

impl<T, E> Publisher for MonoError<T, E> {
  type Item = T;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E>) {
    subscriber.on_subscribe(EmptySubscription);
    subscriber.on_error(self.e);
  }
}
