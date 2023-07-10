use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use std::env;

use super::{AuthData, TokenPayload};

pub struct AuthTokenAdapter {
  pub connection_pool: MySqlPool,
}

impl AuthTokenAdapter {
  pub async fn new() -> Self {
    let db_url = env::var("DATABASE_URL").expect("failed to get database url");

    Self {
      connection_pool: MySqlPool::connect(db_url.as_str())
        .await
        .expect("failed to connect to database"),
    }
  }

  pub async fn select_auth_data(
    &self,
    payload: TokenPayload,
  ) -> Result<AuthData, &'static str> {
    let select_result = sqlx::query!(
      r#"SELECT id, access_token, access_token_valid_until, refresh_token, refresh_token_valid_until FROM auth_data WHERE access_token=? AND refresh_token=?; "#,
      payload.access_token,
      payload.refresh_token,
    )
    .fetch_one(&self.connection_pool)
    .await;

    if select_result.is_err() {
      return Err("auth_data not found");
    }

    let result_object = select_result.unwrap();

    let access_token_valid_until: DateTime<Utc> = DateTime::from_utc(
      result_object.access_token_valid_until.unwrap(), Utc);
    let refresh_token_valid_until: DateTime<Utc> = DateTime::from_utc(
      result_object.refresh_token_valid_until.unwrap(), Utc);

    Ok(AuthData {
      id: result_object.id,
      access_token: result_object.access_token.unwrap(),
      access_token_valid_until: Some(access_token_valid_until),
      refresh_token: result_object.refresh_token.unwrap(),
      refresh_token_valid_until: Some(refresh_token_valid_until),
      created_at: None,
      updated_at: None,
    })
  }
}
