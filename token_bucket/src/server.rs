use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use contract::token_provider_server::{TokenProvider, TokenProviderServer};
use contract::{BookingRequest, BookingResponse};
use token_bucket::bucket_impl::{BucketQueue, RequestToken, TokenBucket, TokenGenerator};

pub mod contract {
  tonic::include_proto!("contract");
}

#[derive(Debug)]
pub struct ContractStruct {
  bucket_queue: Option<Arc<BucketQueue>>,
}

impl Default for ContractStruct {
  fn default() -> Self {
    Self {
      bucket_queue: None,
    }
  }
}

#[tonic::async_trait]
impl TokenProvider for ContractStruct {
  async fn book_token(
    &self,
    request: Request<BookingRequest>,
  ) -> Result<Response<BookingResponse>, Status> {
    println!("request: {:?}", request);

    let token = self.bucket_queue.as_ref().unwrap().issue().unwrap();

    Ok(Response::new(BookingResponse {
      token: token.id,
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;

  let bucket_queue = Arc::new(BucketQueue::new());

  let generator = TokenGenerator::new(&bucket_queue);
  let mut service = ContractStruct::default();

  // bucket_queue.push(RequestToken { id: String::from("token 1") }).unwrap();
  // bucket_queue.push(RequestToken { id: String::from("token 2") }).unwrap();
  // bucket_queue.push(RequestToken { id: String::from("token 3") }).unwrap();

  let _ = tokio::spawn(async move {
    generator.run().await
  });

  service.bucket_queue = Some(bucket_queue);

  Server::builder()
    .add_service(TokenProviderServer::new(service))
    .serve(addr)
    .await?;

  Ok(())
}