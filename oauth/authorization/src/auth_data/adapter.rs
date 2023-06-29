use chrono::Utc;
use sqlx::MySqlPool;

use super::AuthData;

pub struct AuthDataAdapter {
  pub connection_pool: MySqlPool,
}

impl AuthDataAdapter {
  pub async fn new() -> Self {
    Self {
      connection_pool: MySqlPool::connect(&"mysql://innfi:test@127.0.0.1:3306")
        .await
        .expect("failed to connect to database"),
    }
  }

  pub async fn select_auth_data(&self, client_id: String) -> Result<AuthData, &'static str> {
    let select_result = sqlx::query!(
      r#"SELECT client_id, client_secret FROM auth_data WHERE client_id=?;"#,
      client_id,
    )
    .fetch_one(&self.connection_pool)
    .await;

    if select_result.is_err() {
      return Err("auth_data not found")
    }

    let result_object = select_result.unwrap();

    Ok(AuthData {
      id: 0,
      client_id: result_object.client_id.unwrap(),
      client_secret: result_object.client_secret.unwrap(),
      auth_code: String::from(""), //TODO
      auth_code_created_at: Utc::now(),
    })
  }
}