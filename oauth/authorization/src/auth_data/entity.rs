use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
