mod creation;
mod error;
mod filter;
mod foreach;
mod just;
mod misc;
mod schedule_on;
mod spi;
mod transform;

pub use creation::MonoCreate;
pub use error::MonoError;
pub use filter::MonoFilter;
pub use foreach::Foreach;
pub use just::MonoJust;
pub use schedule_on::*;
pub use spi::Mono;
pub use transform::MonoTransform;

pub fn create<T, G, E>(gen: G) -> MonoCreate<T, G, E>
where
  G: Fn() -> Result<T, E>,
{
  MonoCreate::new(gen)
}

pub fn just<T>(t: T) -> MonoJust<T, ()>
where
  T: Clone,
{
  MonoJust::new(t)
}

pub fn error<E>(e: E) -> MonoError<E> {
  MonoError::new(e)
}
