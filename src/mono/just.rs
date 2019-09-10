use crate::mono::Mono;
use crate::spi::{Subscriber, Subscription};
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub struct MonoJust<T, E>
where
  T: Clone,
{
  t: T,
  _e: PhantomData<E>,
}

impl<T, E> MonoJust<T, E>
where
  T: Clone,
{
  pub(crate) fn new(t: T) -> MonoJust<T, E> {
    MonoJust { t, _e: PhantomData }
  }
}

impl<T, E> Mono for MonoJust<T, E>
where
  T: Clone,
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: Subscriber<Item = T, Error = E>,
  {
    let s = Rc::new(subscriber);
    let sub = JustSubscription::new(self.t, s.clone());
    s.on_subscribe(sub);
  }
}

struct JustSubscription<T, S, E>
where
  T: Clone,
  S: Subscriber<Item = T, Error = E>,
{
  value: T,
  subscriber: Rc<S>,
}

impl<T, S, E> JustSubscription<T, S, E>
where
  T: Clone,
  S: Subscriber<Item = T, Error = E>,
{
  fn new(value: T, subscriber: Rc<S>) -> JustSubscription<T, S, E> {
    JustSubscription { value, subscriber }
  }
}

impl<T, S, E> Subscription for JustSubscription<T, S, E>
where
  T: Clone,
  S: Subscriber<Item = T, Error = E>,
{
  fn request(&self, n: usize) {
    if n > 0 {
      self.subscriber.on_next(self.value.clone());
      self.subscriber.on_complete();
    }
  }

  fn cancel(&self) {
    unimplemented!()
  }
}
