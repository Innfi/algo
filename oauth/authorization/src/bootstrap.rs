use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, post};
use log::{info, debug};
use serde::{Deserialize, Serialize};

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
  let server =
    HttpServer::new(move || {
      App::new()
      .route("/", web::get().to(start))
      .service(handle_auth_code)
    })
      .bind("127.0.0.1:8080")?
      .run();

  Ok(server)
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthCodePayload {
  pub client_id: String, //redundant?
  pub id: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthCodeResponse {
  pub msg: String,
  pub auth_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenPayload {
  pub auth_code: String,
  pub client_id: String,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
  pub msg: String,
  pub access_token: String,
  pub refresh_token: String,
}

#[post("/auth")]
async fn handle_auth_code(payload: web::Json<AuthCodePayload>) -> web::Json<AuthCodeResponse> {
  debug!("{:?}", payload);

  let result = AuthCodeResponse {
    msg: String::from("test"),
    auth_code: String::from("auth_code")
  };

  web::Json(result)
}

#[post("/token")]
async fn handle_gen_token(payload: web::Json<TokenPayload>) -> web::Json<TokenResponse> {
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
