mod creation;
mod creation_success;
mod do_finally;
mod do_on_complete;
mod do_on_error;
mod emitter;
mod error;
mod filter;
mod flatmap;
mod foreach;
mod future;
mod just;
mod lazy;
mod misc;
mod processor;
mod schedule_on;
mod spi;
mod transform;
mod transform_error;

pub use creation::MonoCreate;
pub use creation_success::MonoCreateSuccess;
pub use do_finally::MonoDoFinally;
pub use do_on_complete::MonoDoOnComplete;
pub use do_on_error::MonoDoOnError;
pub use emitter::Emitter;
pub use error::MonoError;
pub use filter::MonoFilter;
pub use flatmap::MonoFlatMap;
pub use foreach::Foreach;
pub use just::MonoJust;
pub use lazy::MonoLazy;
pub use processor::Processor;
pub use schedule_on::MonoScheduleOn;
pub use spi::Mono;
pub use transform::MonoTransform;
pub use transform_error::MonoTransformError;

pub fn lazy<T, E, F>(f: F) -> impl Mono<T, E>
where
  F: Fn(&Emitter<T, E>),
{
  MonoLazy::new(f)
}

pub fn success<T, G>(gen: G) -> impl Mono<T, ()>
where
  G: Fn() -> T,
{
  MonoCreateSuccess::new(gen)
}

pub fn create<T, G, E>(gen: G) -> impl Mono<T, E>
where
  G: Fn() -> Result<T, E>,
{
  MonoCreate::new(gen)
}

pub fn just<T, E>(t: T) -> impl Mono<T, E>
where
  T: Clone,
{
  MonoJust::new(t)
}

pub fn error<T, E>(e: E) -> impl Mono<T, E> {
  MonoError::new(e)
}
