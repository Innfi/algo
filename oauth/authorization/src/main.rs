use actix_web::web;
use log::info;
use std::env;

use authorization::auth_data::{AuthDataAdapter, AuthDataService};
use authorization::bootstrap::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  env::set_var("RUST_LOG", "TRACE");
  env_logger::init();

  info!("main()");

  let auth_data_service = create_auth_service_data().await;

  let _ = run_server(auth_data_service)?.await;

  Ok(())
}

async fn create_auth_service_data() -> web::Data<AuthDataService> {
  let auth_adapter = web::Data::new(AuthDataAdapter::new().await);

  web::Data::new(AuthDataService::new(auth_adapter))
}
