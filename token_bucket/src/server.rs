use tonic::{transport::Server, Request, Response, Status};

use contract::token_provider_server::{TokenProvider, TokenProviderServer};
use contract::{BookingRequest, BookingResponse};

pub mod contract {
  tonic::include_proto!("contract");
}

#[derive(Debug, Default)]
pub struct ContractStruct {}

#[tonic::async_trait]
impl TokenProvider for ContractStruct {
  async fn book_token(
    &self,
    request: Request<BookingRequest>,
  ) -> Result<Response<BookingResponse>, Status> {
    println!("request: {:?}", request);

    Ok(Response::new(BookingResponse {
      token: String::from("initial_token")
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let service = ContractStruct::default();

  Server::builder()
    .add_service(TokenProviderServer::new(service))
    .serve(addr)
    .await?;

  Ok(())
}