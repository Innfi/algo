use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct AuthData {
  pub id: i64,
  pub client_id: String,
  pub client_secret: String,
  pub auth_code: String,
  pub auth_code_created_at: DateTime<Utc>,
}
