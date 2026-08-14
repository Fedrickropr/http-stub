use http_stub::{registry::EndpointRegistry, server::Server};
use serde_json::json;

#[tokio::test]
async fn test_get_no_config() {
  let server = Server::new().await;

  let response = reqwest::get(format!("http://{}/hello", server.address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_one_path() {
  let mut registry = EndpointRegistry::new();
  registry.add("GET", "/hello", 400);

  let server = Server::start_from_registry(registry).await;

  let response = reqwest::get(format!("http://{}/hello", server.address))
    .await
    .unwrap();

  assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_multiple_paths() {
  let mut registry = EndpointRegistry::new();
  registry.add("GET", "/hello", 400);
  registry.add("GET", "/goodbye", 401);
  registry.add("GET", "/helloagain", 200);

  let server = Server::start_from_registry(registry).await;

  let response = reqwest::get(format!("http://{}/hello", server.address))
    .await
    .unwrap();

  assert_eq!(response.status(), 400);

  let response = reqwest::get(format!("http://{}/goodbye", server.address))
    .await
    .unwrap();

  assert_eq!(response.status(), 401);

  let response = reqwest::get(format!("http://{}/helloagain", server.address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_add_endpoint() {
  let server = Server::new().await;

  let client = reqwest::Client::new();

  let response = client
    .post(format!("http://{}/__stub/endpoints", server.address))
    .json(&json!({
        "method": "GET",
        "path": "/hello",
        "response_code": 400
    }))
    .send()
    .await
    .unwrap();

  assert_eq!(response.status(), 201);

  let response = client
    .get(format!("http://{}/hello", server.address))
    .send()
    .await
    .unwrap();

  assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_add_del_endpoint() {
  let server = Server::new().await;

  let client = reqwest::Client::new();

  let response = client
    .post(format!("http://{}/__stub/endpoints", server.address))
    .json(&json!({
        "method": "GET",
        "path": "/hello",
        "response_code": 400
    }))
    .send()
    .await
    .unwrap();

  let response = client
    .get(format!("http://{}/hello", server.address))
    .send()
    .await
    .unwrap();

  client
    .delete(format!("http://{}/__stub/endpoints", server.address))
    .json(&json!({
        "method": "GET",
        "path": "/hello",
        "response_code": 200
    }))
    .send()
    .await
    .unwrap();


  let response = client
    .get(format!("http://{}/hello", server.address))
    .send()
    .await
    .unwrap();

  assert_eq!(response.status(), 200);
}
