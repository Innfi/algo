use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use contract::leaky_bucket_sender_server::{LeakyBucketSender, LeakyBucketSenderServer};
use contract::{BucketRequest, SendResponse};

pub mod contract {
  tonic::include_proto!("contract");
}

use leaky_bucket::bucket_impl::{DestinationGateway, LeakyBucket};

pub struct DummySender {}

#[tonic::async_trait]
impl LeakyBucketSender for DummySender {
  async fn send(
    &self, 
    request: Request<BucketRequest>,
  ) -> Result<Response<SendResponse>, Status> {
    println!("request: {:?}", request);

    Ok(Response::new(SendResponse{
      client_id: request.get_ref().client_id.clone(),
      send_result: String::from("default ok"),
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let queue_len: usize = 2000;
  let dest_gateway = Arc::new(DestinationGateway::new());
  let leaky_bucket = LeakyBucket::new(queue_len, dest_gateway);

  let _ = tokio::spawn(async move {
    leaky_bucket.handle_leak().await
  });

  Ok(())
}