use http_stub::{registry::EndpointRegistry, server::Server};
use serde_json::json;

#[tokio::test]
async fn test_get_no_config() {
  let server = Server::new().await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let response = reqwest::get(format!("http://{}/hello", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_one_path() {
  let mut registry = EndpointRegistry::new();
  registry.add("GET", "/hello", 400);

  let server = Server::new_from_registry(registry).await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let response = reqwest::get(format!("http://{}/hello", address))
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

  let server = Server::new_from_registry(registry).await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let response = reqwest::get(format!("http://{}/hello", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 400);

  let response = reqwest::get(format!("http://{}/goodbye", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 401);

  let response = reqwest::get(format!("http://{}/helloagain", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_add_endpoint() {
  let server = Server::new().await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let client = reqwest::Client::new();

  let response = client
    .post(format!("http://{}/__stub/endpoint", address))
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
    .get(format!("http://{}/hello", address))
    .send()
    .await
    .unwrap();

  assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_add_del_endpoint() {
  let server = Server::new().await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let client = reqwest::Client::new();

  client
    .post(format!("http://{}/__stub/endpoint", address))
    .json(&json!({
        "method": "GET",
        "path": "/hello",
        "response_code": 400
    }))
    .send()
    .await
    .unwrap();

  client
    .get(format!("http://{}/hello", address))
    .send()
    .await
    .unwrap();

  client
    .delete(format!("http://{}/__stub/endpoint", address))
    .json(&json!({
        "method": "GET",
        "path": "/hello",
        "response_code": 200
    }))
    .send()
    .await
    .unwrap();

  let response = client
    .get(format!("http://{}/hello", address))
    .send()
    .await
    .unwrap();

  assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_endpoints() {
    let mut registry = EndpointRegistry::new();
    registry.add("GET", "/hello", 200);
    registry.add("POST", "/users", 201);

    let server = Server::new_from_registry(registry).await;
    let address = server.address.clone();

    tokio::spawn(server.run());

    let response = reqwest::get(format!(
        "http://{}/__stub/endpoint",
        address
    ))
    .await
    .unwrap();

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.unwrap();

    assert_eq!(
        body,
        serde_json::json!([
            {
                "method": "GET",
                "path": "/hello",
                "responseCode": 200
            },
            {
                "method": "POST",
                "path": "/users",
                "responseCode": 201
            }
        ])
    );
}
