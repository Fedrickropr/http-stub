use http_stub::registry::*;

#[test]
fn get_empty() {
  let registry = EndpointRegistry::new();

  assert!(registry.get("GET", "/hello").is_none());
}

#[test]
fn get_single() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);

  assert_eq!(
    registry.get("GET", "/hello"),
    Some(&EndpointResponse {
      response_code: 200,
      body: None,
      headers: vec!()
    })
  );
}

#[test]
fn get_with_body() {
  let mut registry = EndpointRegistry::new();

  registry.add_with_string_body("GET", "/hello", 200, "Hello, World!");

  let response = registry.get("GET", "/hello").unwrap();

  assert_eq!(
    response.body,
    Some(ResponseBody::Text("Hello, World!".to_string()))
  );
}

#[test]
fn get_deleted() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);

  registry.delete("GET", "/hello");

  assert!(registry.get("GET", "/hello").is_none());
}

#[test]
fn same_path_different_methods() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  registry.add("POST", "/hello", 201);

  assert_eq!(registry.get("GET", "/hello").unwrap().response_code, 200);

  assert_eq!(registry.get("POST", "/hello").unwrap().response_code, 201);
}

#[test]
fn add_replaces_existing_endpoint() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  registry.add("GET", "/hello", 404);

  assert_eq!(registry.get("GET", "/hello").unwrap().response_code, 404);
}

#[test]
fn add_header() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  let _ = registry.add_header("GET", "/hello", "Content-Type", "application/json");

  let response = registry.get("GET", "/hello").unwrap();

  assert_eq!(
    response.headers,
    vec![ResponseHeader {
      key: "Content-Type".to_string(),
      value: "application/json".to_string(),
    }]
  );
}

#[test]
fn add_multiple_headers() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);

  let _ = registry.add_header("GET", "/hello", "Content-Type", "application/json");
  let _ = registry.add_header("GET", "/hello", "X-Test", "hello");

  let response = registry.get("GET", "/hello").unwrap();

  assert_eq!(response.headers.len(), 2);
}

#[test]
fn add_header_invalid_endpoint() {
  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  let err = registry.add_header("GET", "/not_hello", "Content-Type", "application/json");

  assert_eq!(
    err,
    Err(RegistryError {
      message: "Endpoint not found in registry.".to_string()
    })
  );
}
