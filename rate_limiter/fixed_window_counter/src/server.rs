use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
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
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let counter_tester: usize = 0;
  let counter = Arc::new(Mutex::new(counter_tester));
  let counter_clone = counter.clone();

  tokio::spawn(async move {
    counter_checker(counter_clone).await
  });

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::new("%a %{User-Agent}i"))
      .route("/", web::get().to(dummy_counter_gen))
      .app_data(counter.clone())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

async fn dummy_counter_gen(
  counter: web::Data<Mutex<usize>>,
) -> impl Responder {
  let mut counter_lock = counter.lock().unwrap();

  // println!("dummy_counter_gen] {}", *counter_lock);
  // if *counter_lock >= COUNTER_MAX {
  //   return HttpResponse::TooManyRequests().into();
  // }

  // *counter_lock += 1;

  HttpResponse::Ok().body("hello")
}

async fn counter_checker(counter: Arc<Mutex<usize>>) {
  loop {
    thread::sleep(time::Duration::from_millis(5000));

    let mut lock_guard = counter.lock().unwrap();
    *lock_guard = 0;
    println!("reset")
  }
}