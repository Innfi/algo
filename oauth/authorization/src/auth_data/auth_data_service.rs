use actix_web::web::{self, Data};
use chrono::{DateTime, Duration, Utc};
use log::{debug, error};

use super::AuthDataAdapter;
use crate::entity::{
  AuthCodeResponse, ClientAuthPayload, RegisterClientResponse, TokenPayload,
  TokenResponse,
};

const AUTH_CODE_LIFETIME: i64 = 1000 * 60 * 5;

pub struct AuthDataService {
  auth_adapter: Data<AuthDataAdapter>,
}

impl AuthDataService {
  pub fn new(adapter: Data<AuthDataAdapter>) -> Self {
    Self {
      auth_adapter: adapter.clone(),
    }
  }

  pub async fn register_client(
    &self,
    payload: &ClientAuthPayload,
  ) -> Result<RegisterClientResponse, &'static str> {
    debug!("register_client] payload: {:?}", payload);

    let register_result = self.auth_adapter.insert_auth_data(payload).await;

    if register_result.is_err() {
      error!("register_client] error");
      return Ok(RegisterClientResponse {
        msg: String::from("insert failed"),
      });
    }

    Ok(RegisterClientResponse {
      msg: String::from("success"),
    })
  }

  pub async fn generate_auth_code(
    &self,
    payload: &ClientAuthPayload,
  ) -> Result<AuthCodeResponse, &'static str> {
    debug!("generate_auth_code] payload: {:?}", payload);

    let client_id = payload.client_id.clone();
    let select_result = self.auth_adapter.select_auth_data(&client_id).await;

    if select_result.is_err() {
      error!("generate_auth_code] select failed: {}", client_id);
      return Ok(AuthCodeResponse {
        msg: String::from("no client"),
        auth_code: String::from(""),
        auth_code_valid_until: None,
      });
    }

    let new_auth_code = self.to_new_auth_code(payload);
    let auth_code_valid_until = self.to_new_auth_code_valid_until();

    let update_result = self
      .auth_adapter
      .update_auth_code(&client_id, &new_auth_code, &auth_code_valid_until)
      .await;
    if update_result.is_err() {
      error!("generate_auth_code] update failed: {}", client_id);
      return Ok(AuthCodeResponse {
        msg: String::from("failed to get auth code"),
        auth_code: String::from(""),
        auth_code_valid_until: None,
      });
    }

    Ok(AuthCodeResponse {
      msg: String::from("success"),
      auth_code: new_auth_code,
      auth_code_valid_until: Some(auth_code_valid_until),
    })
  }

  fn to_new_auth_code(&self, payload: &ClientAuthPayload) -> String {
    format!("{}{}", payload.client_id, "1")
  }

  fn to_new_auth_code_valid_until(&self) -> DateTime<Utc> {
    Utc::now() + Duration::seconds(AUTH_CODE_LIFETIME)
  }

  pub async fn generate_token(
    &self,
    payload: &web::Json<TokenPayload>,
  ) -> Result<TokenResponse, &'static str> {
    debug!("payload: {:?}", payload);

    Ok(TokenResponse {
      msg: String::from("success"),
      access_token: String::from("test"),
      access_token_valid_until: None,
      refresh_token: String::from("test"),
      refresh_token_valid_until: None,
    })
  }
}
