use log::info;
use std::env;

use authorization::bootstrap::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "TRACE");
  env_logger::init();

  info!("main()");

  let _ = run_server()?.await;

  Ok(())
}
