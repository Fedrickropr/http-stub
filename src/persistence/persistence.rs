use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

use crate::registry::{EndpointRegistry, ResponseBody};

#[derive(Serialize, Deserialize)]
struct PersistedEndpoint {
    method: String,
    path: String,
    response_code: u16,
    body: Option<ResponseBody>,
}

pub fn save(path: &Path, registry: &EndpointRegistry) -> io::Result<()> {
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

    fs::write(
        path,
        serde_json::to_string_pretty(&endpoints).map_err(io::Error::other)?,
    )
}

pub fn load(path: &Path) -> io::Result<EndpointRegistry> {
    if !path.exists() {
        return Ok(EndpointRegistry::new());
    }

    let endpoints: Vec<PersistedEndpoint> =
        serde_json::from_str(&fs::read_to_string(path)?)
            .map_err(io::Error::other)?;

    let mut registry = EndpointRegistry::new();

    for endpoint in endpoints {
        match endpoint.body {
            Some(ResponseBody::Text(body)) => registry.add_with_string_body(
                &endpoint.method,
                &endpoint.path,
                endpoint.response_code,
                &body,
            ),
            None => registry.add(
                &endpoint.method,
                &endpoint.path,
                endpoint.response_code,
            ),
        }
    }

    Ok(registry)
}
