use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use concurrent_queue::ConcurrentQueue;
use core::time;
use std::{sync::Mutex, thread};
use log;

const COUNTER_MAX: usize = 5;

#[derive(Debug)]
pub struct ReqCountUnit {
  id: usize,
  count: usize,
}

struct WindowHandler {
  mutex_handle: Mutex<ConcurrentQueue<ReqCountUnit>>,
}

impl WindowHandler {
  pub fn new() -> Self {
    Self { 
      mutex_handle: Mutex::new(
        ConcurrentQueue::bounded(COUNTER_MAX*2),
      ),
    }
  }

  pub fn try_increment(&self) -> Result<(), &'static str> {
    todo!();
  }

  pub fn try_reset(&self) -> Result<(), &'static str> {
    todo!();
  }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::new("%a %{User-Agent}i"))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}