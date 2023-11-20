use std::{thread, time};
use ctrlc;
use rand::prelude::*;

use contract::token_provider_client::TokenProviderClient;
use contract::BookingRequest;

pub mod contract {
  tonic::include_proto!("contract");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("client::main] ");

  let mut client = TokenProviderClient::connect("http://[::1]:50051").await?;
  let mut rng = rand::thread_rng();

  loop {
    delay_random(&mut rng);

    let request = tonic::Request::new(BookingRequest {
      client_id: "tester".into(),
    });
    let response = client.book_token(request).await?;

    println!("response={:?}", response);

    let _ = ctrlc::set_handler(move || {
      println!("ctrlc triggered");

      std::process::exit(0);
    });
  }
}

fn delay_random(rng: &mut ThreadRng) {
  let duration: u32 = rng.gen();

  thread::sleep(time::Duration::from_millis((duration as u64)*100))
}