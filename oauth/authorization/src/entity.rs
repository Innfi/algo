use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthCodePayload {
  pub client_id: String, //redundant?
  pub id: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthCodeResponse {
  pub msg: String,
  pub auth_code: String,
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
  pub refresh_token: String,
}
