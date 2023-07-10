use log::info;
use std::env;

use resource::bootstrap::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "TRACE");
  env_logger::init();

  info!("resource] main()");

  let _ = run_server().await?.await;

  Ok(())
}
