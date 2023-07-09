use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterClientResponse {
  pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAuthPayload {
  pub client_id: String, //redundant?
  pub id: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthCodeResponse {
  pub msg: String,
  pub auth_code: String,
  pub auth_code_valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPayload {
  pub auth_code: String,
  pub client_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
  pub msg: String,
  pub access_token: String,
  pub access_token_valid_until: Option<DateTime<Utc>>,
  pub refresh_token: String,
  pub refresh_token_valid_until: Option<DateTime<Utc>>,
}
