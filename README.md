# reactor-rust

[![Crates.io](https://img.shields.io/crates/v/reactor_rs)](https://crates.io/crates/reactor_rs)
[![Crates.io](https://img.shields.io/crates/d/reactor_rs)](https://crates.io/crates/reactor_rs)
[![Build Status](https://travis-ci.com/jjeffcaii/reactor-rust.svg?branch=master)](https://travis-ci.com/jjeffcaii/reactor-rust)
[![License](https://img.shields.io/github/license/jjeffcaii/reactor-rust.svg)](https://github.com/jjeffcaii/reactor-rust/blob/master/LICENSE)
[![GitHub Release](https://img.shields.io/github/release-pre/jjeffcaii/reactor-rust.svg)](https://github.com/jjeffcaii/reactor-rust/releases)

> reactor-rust is an implementation of the [Reactive-Streams](https://www.reactive-streams.org) in Rust.
It is under active development. **Do not use it in a production environment!**

## Install

Add `reactor_rs = 0.0.3` in your `Cargo.toml`.

## Example

> Here are some basic example codes:

### Mono

```rust
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
```
