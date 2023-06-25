use actix_web::web;

use crate::entity::{AuthCodePayload, AuthCodeResponse};

pub struct AuthService {}

impl AuthService {
  pub fn handle_auth_code(
    &self,
    payload: web::Json<AuthCodePayload>,
  ) -> Result<AuthCodeResponse, &'static str> {
    if !self.validate_credential(payload) {
      return Ok(AuthCodeResponse {
        msg: String::from("invalid credential"),
        auth_code: String::from(""),
      });
    }

    Ok(AuthCodeResponse {
      msg: String::from("success"),
      auth_code: String::from("auth_code"),
    })
  }

  fn validate_credential(&self, payload: web::Json<AuthCodePayload>) -> bool {
    if payload.id != String::from("innfi") {
      return false;
    }

    if payload.password != String::from("pass") {
      return false;
    }

    return true;
  }
}
