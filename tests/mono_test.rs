extern crate reactor_rs;

use reactor_rs::mono;
use reactor_rs::prelude::*;
use std::{fmt, marker::PhantomData, thread, time};

#[derive(Debug)]
struct Record {
  name: String,
  age: u8,
}

impl Record {
  fn new(name: String, age: u8) -> Record {
    Record { name, age }
  }
}

struct EchoSubscriber<T, E>
where
  T: fmt::Debug,
  E: fmt::Debug,
{
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E> EchoSubscriber<T, E>
where
  T: fmt::Debug,
  E: fmt::Debug,
{
  fn new() -> EchoSubscriber<T, E> {
    EchoSubscriber {
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E> Subscriber for EchoSubscriber<T, E>
where
  T: fmt::Debug,
  E: fmt::Debug,
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    println!("[ON_COMPLETE]")
  }
  fn on_next(&self, t: T) {
    println!("[ON_NEXT]: {:?}", t);
  }

  fn on_error(&self, e: E) {
    println!("[ON_ERROR]: {:?}", e)
  }
}

#[test]
fn with_error() {
  mono::error("Bad").subscribe(CoreSubscriber::new(
    || (),
    |_it| unreachable!(),
    |e| println!("got err: {}", e),
  ))
}

#[test]
fn tiny() {
  mono::create(|| {
    let ret: Result<u32, ()> = Ok(1234);
    ret
  })
  .subscribe(CoreSubscriber::new(
    || println!("complete!"),
    |it| println!("******* subscribe: {}", it),
    |_| unreachable!(),
  ));
  let just = mono::just(77778888);
  just.clone().subscribe(EchoSubscriber::new());
  just.clone().subscribe(EchoSubscriber::new());
}

#[test]
fn bingo() {
  mono::create(|| {
    let ret: Result<Record, ()> = Ok(Record::new(String::from("Jeffsky"), 18));
    ret
  })
  .do_on_success(|it| println!("******* foreach1: {:?}", it))
  .do_on_success(|it| println!("******* foreach2: {:?}", it))
  .subscribe(EchoSubscriber::new());

  mono::just(1234)
    .do_on_success(|it| println!("******* foreach1: {:?}", it))
    .do_on_success(|it| println!("******* foreach2: {:?}", it))
    .subscribe(EchoSubscriber::new());
}

#[test]
fn test_map() {
  mono::just(2)
    .map(|n| n * 2)
    .map(|n| n + 1)
    .filter(|n| *n > 4)
    .do_on_success(|it| println!("foreach: {:?}", it))
    .filter(|n| *n > 5)
    .subscribe(EchoSubscriber::new());
}

#[test]
fn name() {
  mono::create(|| {
    let result: Result<u32, ()> = Ok(1234);
    result
  })
  .map(move |it| format!("item#{}", it))
  .subscribe(EchoSubscriber::new());
}

#[test]
fn subscribe_on() {
  mono::create(|| {
    let result: Result<u32, ()> = Ok(2);
    result
  })
  .map(|n| n * 2)
  .subscribe_on(Schedulers::new_thread())
  .map(|n| n * 2)
  .do_on_success(|n| println!("bingo: {}@{}", n, thread::current().name().unwrap()))
  .subscribe(Subscribers::noop());
}
