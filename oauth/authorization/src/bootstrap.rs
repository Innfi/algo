use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, post};
use log::{info, debug};
use serde::{Deserialize, Serialize};

/**
 * TODO
 * -----------------------------------------------------------------------------
 * POST /auth -> auth code
 * POST /token
 * 
 * DONE
 * -----------------------------------------------------------------------------
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

#[post("/auth")]
async fn handle_auth_code(payload: web::Json<AuthCodePayload>) -> web::Json<AuthCodeResponse> {
  debug!("{:?}", payload);

  let result = AuthCodeResponse {
    msg: String::from("test"),
    auth_code: String::from("auth_code")
  };

  web::Json(result)
}

async fn start() -> HttpResponse {
  info!("start");

  HttpResponse::Ok().finish()
}
