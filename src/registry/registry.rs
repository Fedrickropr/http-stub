use std::collections::HashMap;

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

  pub fn add(&mut self, method: &str, path: &str, response_code: u16) {
    self.endpoints.insert(
      EndpointKey {
        method: method.to_string(),
        path: path.to_string(),
      },
      EndpointResponse {
        response_code: response_code,
      },
    );
  }

  pub fn delete(&mut self, method: &str, path: &str) {
    self.endpoints.remove(&EndpointKey {
      method: method.to_string(),
      path: path.to_string(),
    });
  }

  pub fn endpoints(&self) -> &HashMap<EndpointKey, EndpointResponse> {
    &self.endpoints
  }
}
