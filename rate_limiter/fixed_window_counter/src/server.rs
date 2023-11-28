use core::time;
use std::{sync::{Mutex, Arc}, thread};
use log;

const COUNTER_MAX: usize = 10;

pub struct Request {}

pub struct Handler {
  counter: usize,
}

impl Handler {
  pub fn new() -> Self {
    Self { counter: 0 }
  }

  pub fn run(&mut self) {
    loop {
      if self.counter >= COUNTER_MAX {
        log::debug!("threshold. continue");
        continue;
      }

      let req = self.receive_request().unwrap();

      self.increment_counter();
      let _ = self.relay_request(req);
    }
  }

  pub fn receive_request(&self) -> Result<Request, &'static str> {
    Ok(Request{}) //FIXME
  }

  fn increment_counter(&mut self) {
    self.counter += 1; //FIXME
  }

  fn relay_request(&self, _req: Request) -> Result<(), &'static str> {

    Ok(())
  }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let counter_tester: u32 = 0;
  let counter = Arc::new(Mutex::new(counter_tester));
  let counter_clone = counter.clone();

  tokio::spawn(async move {
    counter_checker(counter_clone).await
  });

  loop {
    thread::sleep(time::Duration::from_millis(1000));

    let mut counter_mut = counter.lock().unwrap();
    *counter_mut += 1;

    if *counter_mut > 20 {
      break;
    }
  }

  Ok(())
}

async fn counter_checker(counter: Arc<Mutex<u32>>) {
  loop {
    thread::sleep(time::Duration::from_millis(5000));

    let lock_guard = counter.lock().unwrap();
    println!("counter: {}", *lock_guard);
  }
}