use http_stub::cli::parse_port;

#[test]
fn default_port_is_8080() {
  assert_eq!(parse_port(None), 8080);
}

#[test]
fn custom_port_is_used() {
  assert_eq!(parse_port(Some("9000".to_string())), 9000);
}
