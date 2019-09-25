use crate::spi::Subscription;

pub(crate) struct EmptySubscription;

impl Subscription for EmptySubscription {
  fn request(&self, _n: usize) {}
  fn cancel(&self) {}
}
