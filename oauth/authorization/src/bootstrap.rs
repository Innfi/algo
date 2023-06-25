use actix_web::{dev::Server, post, web, App, HttpResponse, HttpServer};
use log::{debug, info};

use crate::auth_service::AuthService;
use crate::entity::{
  AuthCodePayload, AuthCodeResponse, TokenPayload, TokenResponse,
};

/**
 * TODO
 * -----------------------------------------------------------------------------
 * create dummy user
 * create dummy client
 * create dummy resource server
 *
 * DONE
 * -----------------------------------------------------------------------------
 * dummy POST /auth -> auth code
 * dummy POST /token
 */

pub fn run_server() -> Result<Server, std::io::Error> {
  let auth_service = web::Data::new(AuthService {});

  let server = HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(start))
      .service(handle_auth_code)
      .app_data(auth_service.clone())
  })
  .bind("127.0.0.1:8080")?
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
  payload: web::Json<TokenPayload>,
) -> web::Json<TokenResponse> {
  debug!("{:?}", payload);

  let result = TokenResponse {
    msg: String::from("success"),
    access_token: String::from("test_access_token"),
    refresh_token: String::from("test_refresh_token"),
  };

  web::Json(result)
}

async fn start() -> HttpResponse {
  info!("start");

  HttpResponse::Ok().finish()
}
