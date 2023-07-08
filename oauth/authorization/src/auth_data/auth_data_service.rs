use actix_web::web::Data;
use chrono::{DateTime, Duration, Utc};
use log::{debug, error};

use crate::auth_data::adapter::AuthDataAdapter;
use crate::auth_data::entity::{
  ClientAuthPayload, GenerateAuthCodeResponse, RegisterClientResponse,
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
  ) -> Result<GenerateAuthCodeResponse, &'static str> {
    debug!("generate_auth_code] payload: {:?}", payload);

    let client_id = payload.client_id.clone();
    let select_result = self.auth_adapter.select_auth_data(&client_id).await;

    if select_result.is_err() {
      error!("generate_auth_code] select failed: {}", client_id);
      return Ok(GenerateAuthCodeResponse {
        msg: String::from("no client"),
        client_id: Some(client_id),
        auth_code: None,
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
      return Ok(GenerateAuthCodeResponse {
        msg: String::from("failed to get auth code"),
        client_id: Some(client_id),
        auth_code: None,
        auth_code_valid_until: None,
      });
    }

    Ok(GenerateAuthCodeResponse {
      msg: String::from("success"),
      client_id: Some(client_id),
      auth_code: Some(new_auth_code),
      auth_code_valid_until: Some(auth_code_valid_until),
    })
  }

  fn to_new_auth_code(&self, payload: &ClientAuthPayload) -> String {
    format!("{}{}", payload.client_id, "1")
  }

  fn to_new_auth_code_valid_until(&self) -> DateTime<Utc> {
    Utc::now() + Duration::seconds(AUTH_CODE_LIFETIME)
  }
}
