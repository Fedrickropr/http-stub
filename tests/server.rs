use http_stub::{registry::EndpointRegistry, server::Server};

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
