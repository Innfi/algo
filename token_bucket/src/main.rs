use log::info;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "TRACE");
  env_logger::init();

  info!("main] token_bucket");

  Ok(())
}