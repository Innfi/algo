use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use concurrent_queue::ConcurrentQueue;
use core::time;
use std::{sync::Mutex, thread};
use log;

const COUNTER_MAX: f64 = 50.0;
const WINDOW_LEN: usize = 20;

#[derive(Debug)]
pub struct ReqCountUnit {
  id: usize,
  count: usize,
}

#[derive(Debug)]
pub struct SlidingWindow {
  queue: ConcurrentQueue<ReqCountUnit>,
  sum: usize,
  current: usize,
}

struct WindowHandler {
  mutex_handle: Mutex<SlidingWindow>,
}

impl WindowHandler {
  pub fn new() -> Self {
    Self { 
      mutex_handle: Mutex::new(
        SlidingWindow {
          queue: ConcurrentQueue::bounded(WINDOW_LEN* 2),
          sum: 0,
          current: 0,
        },
      ),
    }
  }

  pub fn try_increment(&self) -> Result<(), &'static str> {
    let mut lock = self.mutex_handle.try_lock();

    if let Ok(ref mut mutex) = lock {
      let average: f64 = ((mutex.sum + mutex.current) / WINDOW_LEN) as f64;
      if average > COUNTER_MAX {
        return Err("too_many_request");
      }

      mutex.current += 1;
      return Ok(());
    }

    Err("lock_failed")
  }

  pub fn try_reset(&self) -> Result<(), &'static str> {
    let mut lock = self.mutex_handle.try_lock();

    if let Ok(ref mut mutex) = lock {
      mutex.queue.pop().expect("queue.pop() failed");
      mutex.queue.push(ReqCountUnit {
        id: 0,
        count: mutex.current,
      }).expect("queue.push() failed");

      mutex.sum = mutex.queue
      .try_iter()
      .by_ref()
      .map(|x| x.count)
      .reduce(|a, b| a+b)
      .expect("calculate_sum_failed");

      mutex.current = 0;
    }

    Err("lock_failed")
  }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let window_handler = web::Data::new(WindowHandler::new());
  let handler_clone = window_handler.clone();

  tokio::spawn(async move {
    counter_checker(handler_clone).await
  });

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::new("%a %{User-Agent}i"))
      .route("/", web::get().to(req_handler))
      .app_data(window_handler.clone())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

async fn req_handler(handler: web::Data<WindowHandler>) -> impl Responder {
  let try_incr_result = handler.try_increment();
  if try_incr_result.is_ok() {
    return HttpResponse::Ok().body("hello");
  }

  let err_msg = try_incr_result.unwrap_err();
  if err_msg == "too_many_request" {
    return HttpResponse::TooManyRequests().into();
  }

  HttpResponse::InternalServerError().into()
}

async fn counter_checker(handler: web::Data<WindowHandler>) {
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