use actix_web::{
  dev::Server, get, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_lab::middleware::from_fn;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{middleware::auth_middleware, auth_data::AuthTokenAdapter};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceResponse {
  string_data: String,
  number_data: i64,
}

pub async fn run_server() -> Result<Server, std::io::Error> {
  let auth_adapter = web::Data::new(AuthTokenAdapter::new().await);

  let server = HttpServer::new(move || {
    App::new()
      .wrap(from_fn(auth_middleware))
      .service(dummy_resource)
      .app_data(auth_adapter.clone())
  })
  .bind("0.0.0.0:3000")?
  .run();

  Ok(server)
}

#[get("/{id}")]
async fn dummy_resource(
  auth_adapter: web::Data<AuthTokenAdapter>,
  req: HttpRequest) -> web::Json<ResourceResponse> {
  // debug!("{:?}", payload);
  let id = req.match_info().get("id").expect("id not found");
  debug!("id: {}", id);

  web::Json(ResourceResponse {
    string_data: String::from("resource string"),
    number_data: 1,
  })
}

#[get("/test/{param}")]
async fn another_get(req: HttpRequest) -> Result<HttpResponse, Error> {
  let param = req.match_info().get("param").expect("param not found");
  debug!("param: {}", param);

  Ok(HttpResponse::Ok().json(ResourceResponse {
    string_data: String::from("from /test/param"),
    number_data: 2,
  }))
}
