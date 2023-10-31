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
    // naive approach: toss and forget
    log::debug!("new token: {}", new_token.id);

    self.queue.push(new_token).unwrap();

    return Ok(());
  }
  
  fn issue(&self) -> Result<RequestToken, &'static str> {
    let token = self.queue.pop().unwrap();

    Ok(token)
  }

  fn len(&self) -> usize {
    self.queue.len()
  }
}
