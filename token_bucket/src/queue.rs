use concurrent_queue::ConcurrentQueue;

use crate::token::RequestToken;

pub struct BucketQueue {
  queue: ConcurrentQueue<RequestToken>, 
}

impl BucketQueue {
  pub fn new() -> Self {
    Self {
      queue: ConcurrentQueue::bounded(5),
    }
  }

  pub fn push(&self, new_token: RequestToken) -> Result<(), &'static str> {
    // naive approach: toss and forget

    self.queue.push(new_token).unwrap();

    return Ok(());
  }
}