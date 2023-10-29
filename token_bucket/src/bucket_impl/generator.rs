use std::{thread, time};
use log::debug;

use crate::bucket_impl::TokenBucket;

use super::BucketQueue;
use super::entity::RequestToken;

pub struct TokenGenerator<'a> {
  bucket_queue: &'a BucketQueue,
}

impl<'a> TokenGenerator<'a> {
  pub fn new(queue: &'a BucketQueue) -> Self {
    Self {
      bucket_queue: queue,
    }
  }

  pub async fn run(&self) -> Result<(), &'static str> {
    let mut counter = 0;
    loop {
      debug!("TokenGenerator::run] ");

      thread::sleep(time::Duration::from_millis(5000));

      counter += 1;

      self.bucket_queue.push(RequestToken {
        id: format!("token{}", counter),
      }).unwrap();

      if counter > 100 { //FIXME
        break;
      }
    }

    Ok(())
  }
}

