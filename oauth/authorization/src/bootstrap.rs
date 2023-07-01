use actix_web::{dev::Server, post, web, App, HttpResponse, HttpServer};
use log::{debug, info};

use crate::auth_data::AuthDataService;
use crate::auth_service::AuthService;
use crate::entity::{
  AuthCodePayload, AuthCodeResponse, TokenPayload, TokenResponse,
};

/**
 * TODO
 * -----------------------------------------------------------------------------
 * create dummy resource server
 * create dummy client
 * test with dummy user
 *
 * DONE
 * -----------------------------------------------------------------------------
 * dummy POST /auth -> auth code
 * dummy POST /token
 * service for authorization
 * e2e test
 */

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
  auth_service: web::Data<AuthService>,
  payload: web::Json<AuthCodePayload>,
) -> web::Json<AuthCodeResponse> {
  debug!("{:?}", payload);

  let result = auth_service.handle_auth_code(payload);
  if result.is_err() {}

  web::Json(result.unwrap())
}

#[post("/token")]
async fn handle_gen_token(
  auth_service: web::Data<AuthService>,
  payload: web::Json<TokenPayload>,
) -> web::Json<TokenResponse> {
  debug!("{:?}", payload);

  let result = auth_service.handle_generate_token(payload);
  if result.is_err() {}

  web::Json(result.unwrap())
}

async fn start() -> HttpResponse {
  info!("start");

  HttpResponse::Ok().finish()
}
