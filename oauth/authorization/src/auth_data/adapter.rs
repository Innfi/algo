use log::error;
use sqlx::MySqlPool;
use std::env;

use crate::auth_data::entity::{AuthData, ClientAuthPayload};

pub struct UpdateAuthCodeResult {
  pub msg: String,
}

pub struct AuthDataAdapter {
  pub connection_pool: MySqlPool,
}

impl AuthDataAdapter {
  pub async fn new() -> Self {
    let db_url = env::var("DATABASE_URL").expect("failed to get database url");

    Self {
      connection_pool: MySqlPool::connect(db_url.as_str())
        .await
        .expect("failed to connect to database"),
    }
  }

  pub async fn insert_auth_data(
    &self,
    payload: &ClientAuthPayload,
  ) -> Result<(), &'static str> {
    let insert_result = sqlx::query!(
      r#"INSERT INTO auth_data(client_id, client_pass) VALUES(?, ?);"#,
      payload.client_id,
      payload.client_pass,
    )
    .execute(&self.connection_pool)
    .await;

    if insert_result.is_err() {
      error!("insert_auth_data] error: {}", payload.client_id);
      return Err("insert failed");
    }

    Ok(())
  }

  pub async fn update_auth_code(
    &self,
    client_id: &String,
    auth_code: &String,
  ) -> Result<UpdateAuthCodeResult, &'static str> {
    let update_result = sqlx::query!(
      r#"UPDATE auth_data SET auth_code=? WHERE client_id=?;"#,
      auth_code,
      client_id,
    )
    .execute(&self.connection_pool)
    .await;

    if update_result.is_err() {
      error!("update_auth_code] failed to update: {}", client_id);
      return Err("update failed");
    }

    Ok(UpdateAuthCodeResult {
      msg: String::from("success"),
    })
  }

  pub async fn select_auth_data(
    &self,
    client_id: &String,
  ) -> Result<AuthData, &'static str> {
    let select_result = sqlx::query!(
      r#"SELECT id, client_id, client_pass FROM auth_data WHERE client_id=?;"#,
      client_id,
    )
    .fetch_one(&self.connection_pool)
    .await;

    if select_result.is_err() {
      return Err("auth_data not found");
    }

    let result_object = select_result.unwrap();

    Ok(AuthData {
      id: result_object.id,
      client_id: result_object.client_id,
      client_pass: result_object.client_pass,
      auth_code: String::from(""),
      access_token: String::from(""),
      refresh_token: String::from(""),
      created_at: None,
      updated_at: None,
    })
  }
}
