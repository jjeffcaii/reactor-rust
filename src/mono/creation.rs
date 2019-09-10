use crate::mono::Mono;
use crate::spi::{Subscriber, Subscription};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct MonoCreate<T, G, E>
where
  G: Fn() -> Result<T, E>,
{
  g: G,
}

impl<T, G, E> MonoCreate<T, G, E>
where
  G: Fn() -> Result<T, E>,
{
  pub(crate) fn new(gen: G) -> MonoCreate<T, G, E> {
    MonoCreate { g: gen }
  }
}

impl<T, G, E> Mono for MonoCreate<T, G, E>
where
  G: Fn() -> Result<T, E>,
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: Subscriber<Item = Self::Item, Error = Self::Error>,
  {
    let sub = Rc::new(subscriber);
    let subs = CreateSubscription::new(self.g, sub.clone());
    sub.on_subscribe(subs);
  }
}

struct CreateSubscription<T, G, S, E>
where
  G: Fn() -> Result<T, E>,
  S: Subscriber<Item = T, Error = E>,
{
  g: G,
  actual: Rc<S>,
  requested: Arc<AtomicBool>,
}

impl<T, G, S, E> CreateSubscription<T, G, S, E>
where
  G: Fn() -> Result<T, E>,
  S: Subscriber<Item = T, Error = E>,
{
  fn new(g: G, actual: Rc<S>) -> CreateSubscription<T, G, S, E> {
    CreateSubscription {
      g,
      actual,
      requested: Arc::new(AtomicBool::new(false)),
    }
  }
}

impl<T, G, S, E> Subscription for CreateSubscription<T, G, S, E>
where
  G: Fn() -> Result<T, E>,
  S: Subscriber<Item = T, Error = E>,
{
  fn request(&self, _n: usize) {
    let locker = self.requested.clone();
    if locker.fetch_and(true, Ordering::SeqCst) {
      warn!("subscription has been requested already!");
    } else {
      match (self.g)() {
        Ok(v) => {
          self.actual.on_next(v);
          self.actual.on_complete();
        }
        Err(e) => {
          self.actual.on_error(e);
        }
      }
    }
  }

  fn cancel(&self) {
    unimplemented!()
  }
}
