use http_stub::{management_api::Endpoint, registry::EndpointRegistry, server::Server};
use serde_json::{Value, json};

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

  let response = reqwest::get(format!("http://{}/__stub/endpoint", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);

  let mut body: Vec<Endpoint> = response.json().await.unwrap();

  body.sort_by(|a, b| a.path.cmp(&b.path));

  assert_eq!(
    body,
    vec![
      Endpoint {
        method: "GET".to_string(),
        path: "/hello".to_string(),
        response_code: 200,
        body: None
      },
      Endpoint {
        method: "POST".to_string(),
        path: "/users".to_string(),
        response_code: 201,
        body: None
      },
    ]
  );
}

#[tokio::test]
async fn test_get_endpoints_empty() {
  let server = Server::new().await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let response = reqwest::get(format!("http://{}/__stub/endpoint", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);

  let body: Vec<Endpoint> = response.json().await.unwrap();

  assert!(body.is_empty());
}

#[tokio::test]
async fn test_server_on_specific_port() {
  let server = Server::new_from_registry_on_port(EndpointRegistry::new(), 8080).await;

  assert_eq!(server.address, "127.0.0.1:8080");

  tokio::spawn(async move {
    server.run().await;
  });
}

#[tokio::test]
async fn test_endpoint_response_body() {
  let mut registry = EndpointRegistry::new();
  registry.add("GET", "/hello", 200);
  registry.add("POST", "/users", 201);

  let server = Server::new_from_registry(registry).await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let client = reqwest::Client::new();

  client
    .put(format!("http://{}/__stub/endpoint", address))
    .json(&json!({
        "method": "GET",
        "path": "/hello",
        "response_code": 401,
        "body": "Stubstubstub"
    }))
    .send()
    .await
    .unwrap();

  let response = reqwest::get(format!("http://{}/hello", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 401);

  let body = response.text().await.unwrap();

  assert_eq!(body, "Stubstubstub");
}

#[tokio::test]
async fn export_empty_config() {
  let server = Server::new().await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let response = reqwest::get(format!("http://{}/__stub/config", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);

  let body: Value = response.json().await.unwrap();

  assert_eq!(body, serde_json::json!([]));
}

#[tokio::test]
async fn export_config_with_endpoints() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  registry.add("POST", "/users", 201);

  let server = Server::new_from_registry(registry).await;
  let address = server.address.clone();

  tokio::spawn(server.run());

  let response = reqwest::get(format!("http://{}/__stub/config", address))
    .await
    .unwrap();

  assert_eq!(response.status(), 200);

  let body: Value = response.json().await.unwrap();

  let endpoints = body.as_array().unwrap();

  assert_eq!(endpoints.len(), 2);

  assert!(endpoints.iter().any(|endpoint| {
    endpoint["method"] == "GET" && endpoint["path"] == "/hello" && endpoint["response_code"] == 200
  }));

  assert!(endpoints.iter().any(|endpoint| {
    endpoint["method"] == "POST" && endpoint["path"] == "/users" && endpoint["response_code"] == 201
  }));
}
