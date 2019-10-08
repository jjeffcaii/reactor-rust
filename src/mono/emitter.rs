use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Once;

pub struct Emitter<T, E> {
  tx: Sender<Result<T, E>>,
  once: Once,
}

impl<T, E> Emitter<T, E> {
  pub(crate) fn new(tx: Sender<Result<T, E>>) -> Emitter<T, E> {
    Emitter {
      tx,
      once: Once::new(),
    }
  }

  pub fn success(&self, t: T) {
    let tx = self.tx.clone();
    self.once.call_once(move || {
      tx.send(Ok(t)).unwrap();
    });
  }

  pub fn error(&self, e: E) {
    let tx = self.tx.clone();
    self.once.call_once(move || {
      tx.send(Err(e)).unwrap();
    });
  }
}
