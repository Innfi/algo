use std::sync::Arc;
use std::env;
use tonic::{transport::Server, Request, Response, Status};
use dotenv::dotenv;

use contract::token_provider_server::{TokenProvider, TokenProviderServer};
use contract::{BookingRequest, BookingResponse};
use token_bucket::bucket_impl::{BucketQueue, TokenBucket, TokenGenerator};

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

    let queue = self.bucket_queue.as_ref().unwrap();
    if queue.len() <= 0 {
      return Err(Status::new(
        tonic::Code::Unavailable, 
        String::from("unavailable"),
      ));
    }

    let token = queue.issue().unwrap();

    Ok(Response::new(BookingResponse {
      token: token.id,
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().expect("env load failed");

  let queue_len: usize = env::var("QUEUE_LEN")
    .expect("queue_len not found")
    .parse()
    .expect("queue_len parse failed");
  let duration: u64 = env::var("DURATION")
    .expect("duration not found")
    .parse()
    .expect("duration parse failed");
  let addr = env::var("LISTEN_ADDR").expect("addr not found").parse()?;

  let bucket_queue = Arc::new(BucketQueue::new(queue_len));

  let mut service = ContractStruct::default();
  service.bucket_queue = Some(Arc::clone(&bucket_queue));

  let generator = TokenGenerator::new(bucket_queue);
  let _ = tokio::spawn(async move {
    generator.run(duration, queue_len).await
  });

  Server::builder()
    .add_service(TokenProviderServer::new(service))
    .serve(addr)
    .await?;

  Ok(())
}