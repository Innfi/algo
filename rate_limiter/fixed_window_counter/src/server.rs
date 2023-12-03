use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use core::time;
use std::{sync::Mutex, thread};
use log;

const COUNTER_MAX: usize = 5;

struct CounterHandler {
  mutex_handle: Mutex<usize>,
}

impl CounterHandler {
  pub fn new() -> Self {
    Self { mutex_handle: Mutex::new(0) }
  }

  pub fn try_increment(&self) -> Result<(), &'static str> {
    let mut lock = self.mutex_handle.try_lock();

    if let Ok(ref mut mutex) = lock {
      if **mutex >= COUNTER_MAX {
        return Err("too_many_request");
      }

      **mutex += 1;
      return Ok(());
    }

    Err("lock_failed")
  }

  pub fn try_reset(&self) -> Result<(), &'static str> {
    let mut lock = self.mutex_handle.try_lock();

    if let Ok(ref mut mutex) = lock {
      **mutex = 0;

      return Ok(());
    }

    Err("lock_failed")
  }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let counter_handler = web::Data::new(CounterHandler::new());
  let handler_clone = counter_handler.clone();

  tokio::spawn(async move {
    counter_checker(handler_clone).await
  });

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::new("%a %{User-Agent}i"))
      .route("/", web::get().to(req_handler))
      .app_data(counter_handler.clone())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

async fn req_handler(handler: web::Data<CounterHandler>) -> impl Responder {
  let try_incr_result = handler.try_increment();
  if try_incr_result.is_ok() {
    return HttpResponse::Ok().body("hello");
  }

  let err_msg = try_incr_result.unwrap_err();
  if err_msg == "too_many_request" {
    return HttpResponse::TooManyRequests().into();
  }

  return HttpResponse::Ok().body("lock failed");
}

async fn counter_checker(handler: web::Data<CounterHandler>) {
  loop {
    thread::sleep(time::Duration::from_millis(5000));

    let reset_result = handler.try_reset();
    if reset_result.is_ok() {
      log::info!("counter_checker] reset");
      continue;
    }

    log::info!("try_reset failed");
  }
}