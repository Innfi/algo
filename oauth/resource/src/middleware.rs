use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use actix_web::Error;
use actix_web_lab::middleware::Next;
use log::debug;

pub async fn auth_middleware(
  req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  debug!("uri: {}", req.uri().to_string());

  let auth_header = req
    .headers()
    .get(header::AUTHORIZATION)
    .expect("authorization header not found");

  if !verify_token(auth_header.to_str().unwrap()) {
    return Err(ErrorUnauthorized("unauthorized"));
  }

  next.call(req).await
}

fn verify_token(token: &str) -> bool {
  debug!("token:{}", token);

  if token == &"test_token" {
    return true;
  }

  false
}
