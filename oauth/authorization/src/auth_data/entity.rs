use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAuthPayload {
  pub client_id: String,
  pub client_pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterClientResponse {
  pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateAuthCodeResponse {
  pub msg: String,
  pub client_id: Option<String>,
  pub auth_code: Option<String>,
  pub auth_code_valid_until: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthData {
  pub id: i32,
  pub client_id: String,
  pub client_pass: String,
  pub auth_code: String,
  pub auth_code_valid_until: Option<DateTime<Utc>>,
  pub access_token: String,
  pub access_token_valid_until: Option<DateTime<Utc>>,
  pub refresh_token: String,
  pub refresh_token_valid_until: Option<DateTime<Utc>>,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}
