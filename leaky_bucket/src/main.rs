use std::sync::Arc;

use concurrent_queue::ConcurrentQueue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
  msg: String,
}

pub struct DestinationGateway {
  // TODO
}

impl DestinationGateway {
  pub fn new() -> Self {
    Self {}
  }

  pub fn send(&self, _payload: Payload) -> Result<(), &'static str> {
    // TODO

    Ok(())
  }
}

pub struct LeakyBucket {
  queue: ConcurrentQueue<Payload>,
  gateway: Arc<DestinationGateway>,
}

impl LeakyBucket {
  pub fn new(
      queue_len: usize,
      gw: Arc<DestinationGateway>,
    ) -> Self {
    Self {
      queue: ConcurrentQueue::bounded(queue_len),
      gateway: gw.clone(),
    }
  }

  pub fn push(&self, payload: Payload) -> Result<(), &'static str> {
    if self.queue.is_full() {
      // what is the decision?
      return Ok(())
    }

    self.queue.push(payload).unwrap();

    Ok(())
  }

  pub fn handle_to_be_renamed(&self) {
    
  }
}

fn main() {
  println!("Hello, world!");
}
