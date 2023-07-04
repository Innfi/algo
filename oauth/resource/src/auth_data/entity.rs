use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPayload {
  access_token: String,
  refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthData {
  pub id: i32,
  // pub client_id: String, // don't need these in resource server
  // pub client_pass: String,
  // pub auth_code: String,
  pub access_token: String,
  pub refresh_token: String,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
  //TODO: permission description
}
