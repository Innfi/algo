use std::sync::Arc;
use std::{thread, time};
use uuid::Uuid;

use crate::bucket_impl::TokenBucket;

use super::BucketQueue;
use super::entity::RequestToken;

pub struct TokenGenerator {
  bucket_queue: Arc<BucketQueue>,
}

impl TokenGenerator {
  pub fn new(queue: Arc<BucketQueue>) -> Self {
    Self {
      bucket_queue: Arc::clone(&queue),
    }
  }

  pub async fn run(&self, duration: u64, queue_len: usize) {
    loop {
      println!("TokenGenerator::run] ");
      thread::sleep(time::Duration::from_millis(duration));

      if self.bucket_queue.len() >= queue_len {
        continue;
      }

      self.bucket_queue.push(RequestToken {
        id: self.generate_token(),
      }).unwrap();
    }
  }

  fn generate_token(&self) -> String {
    Uuid::new_v4().to_string()
  }
}

