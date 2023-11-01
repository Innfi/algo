use concurrent_queue::ConcurrentQueue;

use crate::bucket_impl::RequestToken;
use super::QUEUE_LEN;

pub trait TokenBucket {
  fn push(&self, new_token: RequestToken) -> Result<(), &'static str>;
  fn issue(&self) -> Result<RequestToken, &'static str>;
  fn len(&self) -> usize;
}

#[derive(Debug)]
pub struct BucketQueue {
  queue: ConcurrentQueue<RequestToken>, 
}

impl BucketQueue {
  pub fn new() -> Self {
    Self {
      queue: ConcurrentQueue::bounded(QUEUE_LEN),
    }
  }
}

impl TokenBucket for BucketQueue {
  fn push(&self, new_token: RequestToken) -> Result<(), &'static str> {
    // log::debug!("new token: {}", new_token.id);

    if self.queue.is_full() {
      // discard and forget
      return Ok(())
    }

    self.queue.push(new_token).unwrap();

    Ok(())
  }
  
  fn issue(&self) -> Result<RequestToken, &'static str> {
    if self.queue.is_empty() {
      return Err("empty queue");
    }

    let token = self.queue.pop().unwrap();

    Ok(token)
  }

  fn len(&self) -> usize {
    self.queue.len()
  }
}
