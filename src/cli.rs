pub fn parse_port(arg: Option<String>) -> u16 {
  arg.and_then(|arg| arg.parse::<u16>().ok()).unwrap_or(8080)
}
