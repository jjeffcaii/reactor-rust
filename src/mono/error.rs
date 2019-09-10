use super::misc::EmptySubscription;
use super::Mono;
use crate::spi::Subscriber;

pub struct MonoError<E> {
  e: E,
}

impl<E> MonoError<E> {
  pub(crate) fn new(e: E) -> MonoError<E> {
    MonoError { e }
  }
}

impl<E> Mono for MonoError<E> {
  type Item = ();
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: Subscriber<Item = (), Error = E>,
  {
    subscriber.on_subscribe(EmptySubscription);
    subscriber.on_error(self.e);
  }
}
