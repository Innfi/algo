
#[tokio::test]
async fn test_get_id_unauthorized() {
  spawn_app().await;
  let client = reqwest::Client::new();

  let response = client
    .get("http://127.0.0.1:3000/innfi")
    .send()
    .await
    .expect("test_get_id] failed to GET /{id}");

  assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn test_get_id_success() {
  spawn_app().await;
  let client = reqwest::Client::new();

  let response = client
    .get("http://127.0.0.1:3000/innfi")
    .bearer_auth("token_test")
    .send()
    .await
    .expect("test_get_id] failed to GET /{id}");

  assert_eq!(response.status().is_success(), true);
}

async fn spawn_app() {
  let server = resource::bootstrap::run_server().expect("failed to start server");

  let _ = tokio::spawn(server);
}