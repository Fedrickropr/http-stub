use axum::extract::Request;
use axum::{
  Router, extract::State, http::StatusCode, response::IntoResponse, response::Response,
  routing::any,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::registry::EndpointRegistry;
use crate::state::AppState;

pub struct Server {
  pub address: String,
  state: AppState,
  router: Router,
  listener: TcpListener,
}

impl Server {
  pub async fn new() -> Server {
    Self::new_from_registry(EndpointRegistry::new()).await
  }

  pub async fn new_from_registry(registry: EndpointRegistry) -> Server {
    Self::new_from_registry_on_port(registry, 0).await
  }

  pub async fn new_on_port(port: u16) -> Server {
    Self::new_from_registry_on_port(EndpointRegistry::new(), port).await
	}

  pub async fn new_from_registry_on_port(registry: EndpointRegistry, port: u16) -> Server {
    let state = AppState {
      registry: Arc::new(RwLock::new(registry)),
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
      .await
      .unwrap();
    let address = listener.local_addr().unwrap().to_string();
    println!("Server address: {}", address);

    let router = crate::management_api::router()
      .fallback(any(Self::handle_request))
      .with_state(state.clone());

    Self {
      address,
      state,
      router,
      listener,
    }
  }

  pub async fn run(self) {
    axum::serve(self.listener, self.router).await.unwrap();
  }

  async fn handle_request(State(state): State<AppState>, request: Request) -> Response {
    let method = request.method().as_str();
    let path = request.uri().path();

    let registry = state.registry.read().await;

    match registry.get(method, path) {
      Some(response) => (
        StatusCode::from_u16(response.response_code as u16).unwrap(),
        "Hello, World!",
      )
        .into_response(),
      None => (StatusCode::OK, "Hello, World!").into_response(),
    }
  }

  pub async fn stop(&self) {
    !todo!()
  }
}
