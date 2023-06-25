use authorization::entity::{AuthCodePayload, AuthCodeResponse};

#[tokio::test]
async fn test_health_check() {
  spawn_app().await;
  let client = reqwest::Client::new();

  let response = client
    .get("http://127.0.0.1:8080/")
    .send()
    .await
    .expect("test_health_check] failed to check health");

  assert!(response.status().is_success())
}

#[tokio::test]
async fn test_post_auth_success() {
  spawn_app().await;
  let client = reqwest::Client::new();

  let payload = AuthCodePayload {
    client_id: String::from("dummy_id"),
    id: String::from("innfi"),
    password: String::from("pass"),
  };

  let response = client
    .post("http://127.0.0.1:8080/auth")
    .header("Content-Type", "application/json")
    .json(&payload)
    .send()
    .await
    .expect("test_post_auth_success] failed to POST /auth");

  assert!(response.status().is_success());

  let auth_response: AuthCodeResponse =
    response.json().await.expect("json deserialize failed");

  assert_eq!(auth_response.msg, String::from("success"));
  assert_eq!(auth_response.auth_code, String::from("auth_code"));
}

async fn spawn_app() {
  let server =
    authorization::bootstrap::run_server().expect("failed to start server");

  let _ = tokio::spawn(server);
}
