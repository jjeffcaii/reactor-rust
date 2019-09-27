use super::spi::Mono;
use crate::schedulers::Scheduler;
use crate::spi::{Publisher, Subscriber};

pub struct MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<T, E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
  source: M,
  scheduler: C,
}

impl<T, E, M, C> MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<T, E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
  pub fn new(source: M, scheduler: C) -> MonoScheduleOn<T, E, M, C> {
    MonoScheduleOn { source, scheduler }
  }
}

impl<T, E, M, C> Mono<T, E> for MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<T, E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
}

impl<T, E, M, C> Publisher for MonoScheduleOn<T, E, M, C>
where
  M: 'static + Send + Mono<T, E> + Sized,
  C: Scheduler<Item = T, Error = E> + Sized,
{
  type Item = T;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E> + 'static + Send) {
    self.scheduler.schedule(self.source, subscriber);
  }
}
