use std::sync::Arc;
use std::{thread, time};
use uuid::Uuid;

use crate::bucket_impl::TokenBucket;

use super::{BucketQueue, QUEUE_LEN};
use super::entity::RequestToken;

const DURATION: u64 = 5000;

pub struct TokenGenerator {
  bucket_queue: Arc<BucketQueue>,
}

impl TokenGenerator {
  pub fn new(queue: Arc<BucketQueue>) -> Self {
    Self {
      bucket_queue: Arc::clone(&queue),
    }
  }

  pub async fn run(&self) {
    loop {
      println!("TokenGenerator::run] ");
      thread::sleep(time::Duration::from_millis(DURATION));

      if self.bucket_queue.len() >= QUEUE_LEN {
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

