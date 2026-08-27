use http_stub::{
  persistence,
  registry::{EndpointRegistry, EndpointResponse, ResponseBody},
};
use tempfile::tempdir;

#[test]
fn save_and_load_empty_registry() {
  let dir = tempdir().unwrap();
  let path = dir.path().join("stub.json");

  let registry = EndpointRegistry::new();

  persistence::save(&path, &registry).unwrap();
  let loaded = persistence::load(&path).unwrap();

  assert!(loaded.endpoints().is_empty());
}

#[test]
fn save_and_load_many_endpoints() {
  let dir = tempdir().unwrap();
  let path = dir.path().join("stub.json");

  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  registry.add("POST", "/users", 201);
  registry.add("DELETE", "/users/1", 204);

  persistence::save(&path, &registry).unwrap();
  let loaded = persistence::load(&path).unwrap();

  assert_eq!(loaded.endpoints().len(), 3);

  assert_eq!(
    loaded.get("GET", "/hello"),
    Some(&EndpointResponse {
      response_code: 200,
      body: None,
			headers: vec!()
    })
  );

  assert_eq!(
    loaded.get("POST", "/users"),
    Some(&EndpointResponse {
      response_code: 201,
      body: None,
			headers: vec!()
    })
  );

  assert_eq!(
    loaded.get("DELETE", "/users/1"),
    Some(&EndpointResponse {
      response_code: 204,
      body: None,
			headers: vec!()
    })
  );
}

#[test]
fn save_and_load_endpoint_with_body() {
  let dir = tempdir().unwrap();
  let path = dir.path().join("stub.json");

  let mut registry = EndpointRegistry::new();
  registry.add_with_string_body("GET", "/hello", 200, "Hello world");

  persistence::save(&path, &registry).unwrap();
  let loaded = persistence::load(&path).unwrap();

  assert_eq!(
    loaded.get("GET", "/hello"),
    Some(&EndpointResponse {
      response_code: 200,
      body: Some(ResponseBody::Text("Hello world".to_string())),
			headers: vec!()
    })
  );
}

#[test]
fn deleted_endpoint_is_not_persisted() {
  let dir = tempdir().unwrap();
  let path = dir.path().join("stub.json");

  let mut registry = EndpointRegistry::new();

  registry.add("GET", "/hello", 200);
  registry.add("GET", "/goodbye", 200);
  registry.delete("GET", "/hello");

  persistence::save(&path, &registry).unwrap();
  let loaded = persistence::load(&path).unwrap();

  assert!(loaded.get("GET", "/hello").is_none());
  assert!(loaded.get("GET", "/goodbye").is_some());
}
