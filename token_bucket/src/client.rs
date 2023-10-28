use contract::token_provider_client::TokenProviderClient;
use contract::BookingRequest;

pub mod contract {
  tonic::include_proto!("contract");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("client::main] ");

  let mut client = TokenProviderClient::connect("http://[::1]:50051").await?;

  let request = tonic::Request::new(BookingRequest {
    client_id: "tester".into(),
  });
  let response = client.book_token(request).await?;

  println!("response={:?}", response);

  Ok(())
}