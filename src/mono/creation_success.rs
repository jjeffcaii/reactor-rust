use super::Mono;
use crate::spi::{Subscriber, Subscription};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct MonoCreateSuccess<T, G>
where
  G: Fn() -> T,
{
  g: G,
}

impl<T, G> MonoCreateSuccess<T, G>
where
  G: Fn() -> T,
{
  pub(crate) fn new(gen: G) -> MonoCreateSuccess<T, G> {
    MonoCreateSuccess { g: gen }
  }
}

impl<T, G> Mono for MonoCreateSuccess<T, G>
where
  G: Fn() -> T,
{
  type Item = T;
  type Error = ();

  fn subscribe<S>(self, subscriber: S)
  where
    S: Subscriber<Item = T, Error = ()>,
  {
    let sub = Rc::new(subscriber);
    let subs = CreateSuccessSubscription::new(self.g, sub.clone());
    sub.on_subscribe(subs);
  }
}

struct CreateSuccessSubscription<T, G, S>
where
  G: Fn() -> T,
  S: Subscriber<Item = T, Error = ()>,
{
  g: G,
  actual: Rc<S>,
  requested: Arc<AtomicBool>,
}

impl<T, G, S> CreateSuccessSubscription<T, G, S>
where
  G: Fn() -> T,
  S: Subscriber<Item = T, Error = ()>,
{
  fn new(g: G, actual: Rc<S>) -> CreateSuccessSubscription<T, G, S> {
    CreateSuccessSubscription {
      g,
      actual,
      requested: Arc::new(AtomicBool::new(false)),
    }
  }
}

impl<T, G, S> Subscription for CreateSuccessSubscription<T, G, S>
where
  G: Fn() -> T,
  S: Subscriber<Item = T, Error = ()>,
{
  fn request(&self, _n: usize) {
    let locker = self.requested.clone();
    if locker.fetch_and(true, Ordering::SeqCst) {
      warn!("subscription has been requested already!");
    } else {
      let v = (self.g)();
      self.actual.on_next(v);
      self.actual.on_complete();
    }
  }

  fn cancel(&self) {
    unimplemented!()
  }
}
