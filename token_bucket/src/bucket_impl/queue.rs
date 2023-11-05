use concurrent_queue::ConcurrentQueue;

use crate::bucket_impl::RequestToken;

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
  pub fn new(queue_len: usize) -> Self {
    Self {
      queue: ConcurrentQueue::bounded(queue_len),
    }
  }
}

impl TokenBucket for BucketQueue {
  fn push(&self, new_token: RequestToken) -> Result<(), &'static str> {
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
