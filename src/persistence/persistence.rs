use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::registry::{EndpointRegistry, ResponseBody};

#[derive(Serialize, Deserialize)]
struct PersistedEndpoint {
  method: String,
  path: String,
  response_code: u16,
  body: Option<ResponseBody>,
}

pub fn serialize(registry: &EndpointRegistry) -> Result<String, serde_json::Error> {
  let endpoints: Vec<_> = registry
    .endpoints()
    .iter()
    .map(|(key, response)| PersistedEndpoint {
      method: key.method.clone(),
      path: key.path.clone(),
      response_code: response.response_code,
      body: response.body.clone(),
    })
    .collect();

  serde_json::to_string_pretty(&endpoints)
}

pub fn deserialize(data: &str) -> Result<EndpointRegistry, serde_json::Error> {
  let endpoints: Vec<PersistedEndpoint> = serde_json::from_str(data)?;

  let mut registry = EndpointRegistry::new();

  for endpoint in endpoints {
    match endpoint.body {
      Some(ResponseBody::Text(body)) => registry.add_with_string_body(
        &endpoint.method,
        &endpoint.path,
        endpoint.response_code,
        &body,
      ),
      None => registry.add(&endpoint.method, &endpoint.path, endpoint.response_code),
    }
  }

  Ok(registry)
}

pub fn save(path: &Path, registry: &EndpointRegistry) -> io::Result<()> {
  let json = serialize(registry).map_err(io::Error::other)?;
  fs::write(path, json)
}

pub fn load(path: &Path) -> io::Result<EndpointRegistry> {
  let json = fs::read_to_string(path)?;
  deserialize(&json).map_err(io::Error::other)
}
