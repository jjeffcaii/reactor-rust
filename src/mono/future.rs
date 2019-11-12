use crate::spi::Publisher;
use std::future::Future;
use std::task::{Context, Poll};

struct ToFuture<P, T, E>
where
  P: Publisher<Item = T, Error = E>,
{
  source: P,
}

impl<P, T, E> ToFuture<P, T, E>
where
  P: Publisher<Item = T, Error = E>,
{
  fn new(source: P) -> ToFuture<P, T, E> {
    ToFuture { source }
  }
}
