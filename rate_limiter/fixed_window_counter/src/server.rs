use actix_web::{web, App, HttpResponse, HttpServer};
use core::time;
use std::{sync::{Mutex, Arc}, thread};
use log;

const COUNTER_MAX: u32 = 10;

pub struct Request {}

pub struct Handler {
  counter: u32,
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

  // loop {
  //   thread::sleep(time::Duration::from_millis(1000));

  //   let mut counter_mut = counter.lock().unwrap();
  //   *counter_mut += 1;

  //   if *counter_mut > 20 {
  //     break;
  //   }
  // }

  let _ = HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(dummy_couter_gen))
      .app_data(counter.clone())
  })
  .bind("localhost:8080")?
  .run()
  .await;

  Ok(())
}

async fn dummy_couter_gen(
  //counter: Arc<Mutex<u32>>
  counter: web::Data<Mutex<u32>>,
) -> HttpResponse {
  let mut counter_lock = counter.try_lock().unwrap();

  println!("dummy_counter_gen] {}", *counter_lock);
  if *counter_lock >= 10 {
    return HttpResponse::TooManyRequests().into();
  }

  *counter_lock += 1;

  HttpResponse::Ok().into()
}

async fn counter_checker(counter: Arc<Mutex<u32>>) {
  loop {
    thread::sleep(time::Duration::from_millis(5000));

    let mut lock_guard = counter.lock().unwrap();
    *lock_guard = 0;
    println!("reset")
  }
}