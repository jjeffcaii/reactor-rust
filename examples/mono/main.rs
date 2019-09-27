extern crate reactor_rs;

use reactor_rs::mono;
use reactor_rs::prelude::*;
use reactor_rs::schedulers;
use std::{thread, time::Duration};

fn main() {
  let result = mono::just(42)
    .do_on_success(|n| {
      println!(
        "Answer to the Ultimate Question of Life, The Universe, and Everything: {}",
        n
      );
    })
    .subscribe_on(schedulers::new_thread())
    .flatmap(|n| {
      // flatmap async and sleep 500ms.
      mono::success(move || {
        thread::sleep(Duration::from_millis(500));
        n * 2
      })
      .subscribe_on(schedulers::new_thread())
    })
    .map(|n| n * 2)
    .block()
    .unwrap()
    .unwrap();
  println!("now it should be {}: actual={}!", 42 * 4, result);
}
