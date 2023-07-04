use actix_web::{dev::Server, web, App, HttpServer};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::auth_data::TokenPayload;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceResponse {
  string_data: String,
  number_data: i64,
}

pub fn run_server() -> Result<Server, std::io::Error> {
  let server = HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(dummy_resource))
  })
  .bind("0.0.0.0:3000")?
  .run();

  Ok(server)
}

async fn dummy_resource(payload: web::Json<TokenPayload>) -> web::Json<ResourceResponse> {
  debug!("{:?}", payload);

  web::Json(ResourceResponse {
    string_data: String::from("resource string"),
    number_data: 1,
  })
}