use std::sync::Arc;
use tokio::sync::RwLock;

use crate::registry::EndpointRegistry;

#[derive(Clone)]
pub struct AppState {
  pub registry: Arc<RwLock<EndpointRegistry>>,
}
