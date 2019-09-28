extern crate reactor_rs;

use reactor_rs::mono;
use reactor_rs::prelude::*;
use reactor_rs::schedulers;
use std::{fmt, marker::PhantomData};
use std::{thread, time::Duration};

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

  fn on_subscribe(&self, subscription: impl Subscription) {
    subscription.request(REQUEST_MAX)
  }

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
  mono::error("THIS_IS_A_MOCK_ERROR")
    .do_on_error(|e| println!("DO_ON_ERROR: {}", e))
    .subscribe(Subscribers::noop())
}

#[test]
fn tiny() {
  mono::create(|| {
    let ret: Result<u32, ()> = Ok(1234);
    ret
  })
  .subscribe(EchoSubscriber::new());
  let just = mono::just(77778888);
  just.clone().subscribe(EchoSubscriber::new());
  just.clone().subscribe(EchoSubscriber::new());
}

#[test]
fn test_next_subscriber() {
  mono::success(|| 42).subscribe(Subscribers::next(|v| assert_eq!(42, v)));
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
    .do_on_success(|n| assert_eq!(5, *n))
    .filter(|n| *n > 5)
    .subscribe(EchoSubscriber::new());
}

#[test]
fn create_success() {
  mono::success(|| 1234)
    .map(|it| format!("item#{}", it))
    .subscribe(EchoSubscriber::new());
}

#[test]
fn subscribe_on() {
  mono::success(|| 2)
    .map(|n| n * 2)
    .subscribe_on(schedulers::new_thread())
    .map(|n| n * 2)
    .do_on_success(|n| assert_eq!(8, *n))
    .subscribe(Subscribers::noop());
}

#[test]
fn block() {
  let v = mono::just(512)
    .subscribe_on(schedulers::new_thread())
    .map(|it| {
      thread::sleep(Duration::from_secs(1));
      it * 2
    })
    .block()
    .unwrap()
    .unwrap();
  assert_eq!(1024, v);
}

#[test]
fn test_flatmap() {
  let result = mono::just(1)
    .flatmap(|n| {
      mono::success(move || {
        thread::sleep(Duration::from_millis(200));
        n * 2
      })
      .subscribe_on(schedulers::new_thread())
    })
    .block()
    .unwrap()
    .unwrap();
  println!("----> result: {}", result);
  assert_eq!(2, result);
}

#[test]
fn test_finally() {
  mono::success(|| 1234)
    .do_finally(|| {
      println!("====> DO_FINALLY!!!");
    })
    .map(|v| format!("Hello {}", v))
    .do_on_complete(|| println!("====> call do_on_complet!"))
    .subscribe(EchoSubscriber::new());
  mono::error("Oops!")
    .do_finally(|| {
      println!("====> DO_FINALLY!!!");
    })
    .do_on_complete(|| unreachable!())
    .subscribe(EchoSubscriber::new());
}
