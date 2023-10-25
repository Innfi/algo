use concurrent_queue::ConcurrentQueue;

use crate::token::RequestToken;

pub trait TokenBucket {
  fn push(&self, new_token: RequestToken) -> Result<(), &'static str>;
  fn issue(&self) -> Result<RequestToken, &'static str>;
}

pub struct BucketQueue {
  queue: ConcurrentQueue<RequestToken>, 
}

impl BucketQueue {
  pub fn new() -> Self {
    Self {
      queue: ConcurrentQueue::bounded(5),
    }
  }
}

impl TokenBucket for BucketQueue {
  fn push(&self, new_token: RequestToken) -> Result<(), &'static str> {
    // naive approach: toss and forget

    self.queue.push(new_token).unwrap();

    return Ok(());
  }
  
  fn issue(&self) -> Result<RequestToken, &'static str> {
    todo!();
  }
}
