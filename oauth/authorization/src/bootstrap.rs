use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use log::info;

pub fn run_server() -> Result<Server, std::io::Error> {
  let server =
    HttpServer::new(move || App::new().route("/", web::get().to(start)))
      .bind("127.0.0.1:8080")?
      .run();

  Ok(server)
}

async fn start() -> HttpResponse {
  info!("start");

  HttpResponse::Ok().finish()
}
