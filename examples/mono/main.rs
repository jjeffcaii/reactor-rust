extern crate reactor_rs;

use reactor_rs::mono;
use reactor_rs::prelude::*;
use reactor_rs::schedulers;
use std::{thread, time::Duration};

fn main() {
  mono::just(42)
    .do_on_success(|n| {
      println!(
        "Answer to the Ultimate Question of Life, The Universe, and Everything: {}",
        n
      );
    })
    .subscribe_on(schedulers::new_thread())
    .map(|n| n * 2)
    .subscribe(Subscribers::next(|n| {
      println!("now it should be 84: actual={}!", n)
    }));
  // waiting 1s
  thread::sleep(Duration::from_secs(1));
}
