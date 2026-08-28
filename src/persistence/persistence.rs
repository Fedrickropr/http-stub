use std::{fs, io, path::Path};

use crate::{
  management_api::Endpoint,
  registry::{EndpointRegistry, ResponseBody},
};

pub fn serialize(registry: &EndpointRegistry) -> Result<String, serde_json::Error> {
  let endpoints: Vec<_> = registry
    .endpoints()
    .iter()
    .map(|(key, response)| Endpoint {
      method: key.method.clone(),
      path: key.path.clone(),
      response_code: response.response_code,
      body: response.body.clone(),
      headers: response.headers.clone(),
    })
    .collect();

  serde_json::to_string_pretty(&endpoints)
}

pub fn deserialize(data: &str) -> Result<EndpointRegistry, serde_json::Error> {
  let endpoints: Vec<Endpoint> = serde_json::from_str(data)?;

  let mut registry = EndpointRegistry::new();

  for endpoint in endpoints {
    match endpoint.body {
      Some(ResponseBody::Text(body)) => {
        registry.add_with_string_body(
          &endpoint.method,
          &endpoint.path,
          endpoint.response_code,
          &body,
        );
        for header in endpoint.headers {
          // Ignore - can't error
          let _ = registry.add_header(&endpoint.method, &endpoint.path, &header.key, &header.value);
        }
      }
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
