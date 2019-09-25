# reactor-rust

> reactor-rust is an implementation of the [Reactive-Streams](https://www.reactive-streams.org) in Rust.  
It is under active development. **Do not use it in a production environment!**

## Install

Add `reactor_rs = 0.0.1` in your `Cargo.toml`.

## Example

> Here are some basic example codes:

### Mono

```rust
extern crate reactor_rs;

use reactor_rs::mono;
use reactor_rs::prelude::*;
use std::{thread, time::Duration};

fn main() {
  mono::just(42)
    .do_on_success(|n| {
      println!(
        "Answer to the Ultimate Question of Life, The Universe, and Everything: {}",
        n
      );
    })
    .subscribe_on(Schedulers::new_thread())
    .map(|n| n * 2)
    .subscribe(CoreSubscriber::new(
      || println!("on complete"),
      |n| println!("now it should be 84: actual={}!", n),
      |_e| unreachable!(),
    ));
  // waiting 1s
  thread::sleep(Duration::from_secs(1));
}
```
