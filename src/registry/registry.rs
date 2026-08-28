use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct EndpointRegistry {
  endpoints: HashMap<EndpointKey, EndpointResponse>,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct EndpointKey {
  pub method: String,
  pub path: String,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct EndpointResponse {
  pub response_code: u16,
  pub body: Option<ResponseBody>,
  pub headers: Vec<ResponseHeader>,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub enum ResponseBody {
  Text(String),
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHeader {
  pub key: String,
  pub value: String,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Serialize, Deserialize)]
pub struct RegistryError {
  pub message: String,
}

impl EndpointRegistry {
  pub fn new() -> Self {
    Self {
      endpoints: HashMap::new(),
    }
  }

  pub fn get(&self, method: &str, path: &str) -> Option<&EndpointResponse> {
    let key = EndpointKey {
      method: method.to_string(),
      path: path.to_string(),
    };

    return self.endpoints.get(&key);
  }

  pub fn endpoints(&self) -> &HashMap<EndpointKey, EndpointResponse> {
    &self.endpoints
  }

  pub fn add_with_string_body(&mut self, method: &str, path: &str, response_code: u16, body: &str) {
		let key = EndpointKey {
        method: method.to_string(),
        path: path.to_string()
      };

		match self.endpoints.get_mut(&key) {
			Some(endpoint) => {
				endpoint.body = Some(ResponseBody::Text(body.to_string()));
				endpoint.response_code = response_code;
			},
			None => _ = self.endpoints.insert(
				EndpointKey {
					method: method.to_string(),
					path: path.to_string(),
				},
				EndpointResponse {
					response_code: response_code,
					body: Some(ResponseBody::Text(body.to_string())),
					headers: Vec::new()
				})
		};
  }

  pub fn add(&mut self, method: &str, path: &str, response_code: u16) {
    self.endpoints.insert(
      EndpointKey {
        method: method.to_string(),
        path: path.to_string(),
      },
      EndpointResponse {
        response_code: response_code,
        body: None,
        headers: Vec::new(),
      },
    );
  }

  pub fn add_header(
    &mut self,
    method: &str,
    path: &str,
    key: &str,
    value: &str,
  ) -> Result<(), RegistryError> {
    let endpoint_key = EndpointKey {
      method: method.to_string(),
      path: path.to_string(),
    };

    let response = match self.endpoints.get_mut(&endpoint_key) {
      Some(response) => response,
      None => Err(RegistryError {
        message: "Endpoint not found in registry.".to_string(),
      })?,
    };

    response.headers.push(ResponseHeader {
      key: key.to_string(),
      value: value.to_string(),
    });

    Ok(())
  }

  pub fn delete_header(
    &mut self,
    method: &str,
    path: &str,
    key: &str,
  ) -> Result<(), RegistryError> {
    let endpoint_key = EndpointKey {
      method: method.to_string(),
      path: path.to_string(),
    };

    let response = match self.endpoints.get_mut(&endpoint_key) {
      Some(response) => response,
      None => Err(RegistryError {
        message: "Endpoint not found in registry.".to_string(),
      })?,
    };

		let index = response.headers.iter().position(|x| x.key == key).unwrap();
		response.headers.remove(index);

    Ok(())
  }

  pub fn delete(&mut self, method: &str, path: &str) {
    self.endpoints.remove(&EndpointKey {
      method: method.to_string(),
      path: path.to_string(),
    });
  }
}
