use actix_web::{dev::Server, post, web, App, HttpResponse, HttpServer};
use log::{debug, info};

use crate::{
  auth_data::AuthDataService,
  entity::{AuthCodeResponse, ClientAuthPayload, TokenPayload, TokenResponse},
};

pub fn run_server(
  auth_service: web::Data<AuthDataService>,
) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(start))
      .service(handle_auth_code)
      .service(handle_gen_token)
      .app_data(auth_service.clone())
  })
  .bind("0.0.0.0:8080")?
  .run();

  Ok(server)
}

#[post("/auth")]
async fn handle_auth_code(
  auth_service: web::Data<AuthDataService>,
  payload: web::Json<ClientAuthPayload>,
) -> web::Json<AuthCodeResponse> {
  debug!("{:?}", payload);

  let result = auth_service.generate_auth_code(&payload).await;
  if result.is_err() {}

  web::Json(result.unwrap())
}

#[post("/token")]
async fn handle_gen_token(
  auth_service: web::Data<AuthDataService>,
  payload: web::Json<TokenPayload>,
) -> web::Json<TokenResponse> {
  debug!("{:?}", payload);

  let result = auth_service.generate_token(&payload).await;
  if result.is_err() {}

  web::Json(result.unwrap())
}

async fn start() -> HttpResponse {
  info!("start");

  HttpResponse::Ok().finish()
}
