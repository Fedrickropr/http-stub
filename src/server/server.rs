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
}

impl Server {
  pub async fn start_from_registry(registry: EndpointRegistry) -> Server {
    let state = AppState {
      registry: Arc::new(RwLock::new(registry)),
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap().to_string();
    println!("Server address: {}", address);

    let router = crate::management_api::router()
      .fallback(any(Self::handle_request))
      .with_state(state.clone());

    tokio::spawn(async move {
      axum::serve(listener, router).await.unwrap();
    });

    Self { address, state }
  }

  pub async fn new() -> Server {
    Self::start_from_registry(EndpointRegistry::new()).await
  }

  async fn handle_request(State(state): State<AppState>, request: Request) -> Response {
    let method = request.method().as_str();
    let path = request.uri().path();

    let registry = state.registry.read().await;

    match registry.get(method, path) {
      Some(endpoint) => (
        StatusCode::from_u16(endpoint.response_code as u16).unwrap(),
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
