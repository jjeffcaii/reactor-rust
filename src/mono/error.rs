use super::misc::EmptySubscription;
use super::spi::Mono;
use crate::spi::{Publisher, Subscriber};

pub struct MonoError<E> {
  e: E,
}

impl<E> MonoError<E> {
  pub(crate) fn new(e: E) -> MonoError<E> {
    MonoError { e }
  }
}

impl<E> Mono<(), E> for MonoError<E> {}

impl<E> Publisher for MonoError<E> {
  type Item = ();
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = (), Error = E>) {
    subscriber.on_subscribe(EmptySubscription);
    subscriber.on_error(self.e);
  }
}
