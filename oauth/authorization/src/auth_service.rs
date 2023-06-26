use actix_web::web;

use crate::entity::{AuthCodePayload, AuthCodeResponse, TokenResponse, TokenPayload};

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

    true
  }

  pub fn handle_generate_token(&self, payload: web::Json<TokenPayload>) -> Result<TokenResponse, &'static str> {
    if !self.validate_auth_code(payload) {
      return Ok(TokenResponse {
        msg: String::from("invalid auth code"),
        access_token: String::from(""),
        refresh_token: String::from(""),
      });
    }

    Ok(TokenResponse {
      msg: String::from("success"),
      access_token: String::from("dummy_access_token"),
      refresh_token: String::from("dummy_refresh_token")
    })
  }

  fn validate_auth_code(&self, payload: web::Json<TokenPayload>) -> bool {
    if payload.auth_code != String::from("auth_code") {
      return false;
    }

    true
  }
}
