use std::{sync::Arc, thread, time};

use concurrent_queue::ConcurrentQueue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
  msg: String,
}

pub struct DestinationGateway {
  // TODO: interface for destination (web, protobuf, etc)
}

impl DestinationGateway {
  pub fn new() -> Self {
    Self {}
  }

  pub fn send(&self, payload: Payload) -> Result<(), &'static str> {
    // TODO

    println!("DestincationGateway.send] {:?}", payload);

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

  pub async fn handle_leak(&self) {
    // FIXME
    let duration: u64 = 5000;
    let leak_len: usize = 10;

    loop {
      println!("LeakyBucket::handle] ");

      for _ in 0..leak_len {
        if self.queue.is_empty() {
          break;
        }

        let payload = self.queue.pop().unwrap();
        let _ = self.gateway.send(payload);
      }

      thread::sleep(time::Duration::from_millis(duration));
    }
  }
}
