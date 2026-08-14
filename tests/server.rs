use http_stub::{management_api::Endpoint, registry::EndpointRegistry, server::Server};
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
      },
      Endpoint {
        method: "POST".to_string(),
        path: "/users".to_string(),
        response_code: 201,
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
