mod tests {
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
      Some(&EndpointResponse { response_code: 200 })
    );
  }

  #[test]
  fn get_deleted() {
    let mut registry = EndpointRegistry::new();

    registry.add("GET", "/hello", 200);

    registry.delete("GET", "/hello");

    assert!(registry.get("GET", "/hello").is_none());
  }
}
