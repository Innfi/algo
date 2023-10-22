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
}