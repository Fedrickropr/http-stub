use axum::{
  Json, Router,
  extract::State,
  http::StatusCode,
  routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Deserialize)]
struct AddEndpointRequest {
  method: String,
  path: String,
  response_code: u16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
  pub path: String,
  pub method: String,
  pub response_code: u16,
}

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/__stub/endpoint", get(get_endpoint))
    .route("/__stub/endpoint", post(add_endpoint))
    .route("/__stub/endpoint", delete(delete_endpoint))
}

async fn get_endpoint(State(state): State<AppState>) -> Json<Vec<Endpoint>> {
  let registry = state.registry.read().await;

  let endpoints = registry
    .endpoints()
    .iter()
    .map(|(key, response)| Endpoint {
      method: key.method.clone(),
      path: key.path.clone(),
      response_code: response.response_code,
    })
    .collect();

  Json(endpoints)
}

async fn add_endpoint(
  State(state): State<AppState>,
  Json(request): Json<AddEndpointRequest>,
) -> StatusCode {
  let mut registry = state.registry.write().await;

  registry.add(&request.method, &request.path, request.response_code);

  StatusCode::CREATED
}

async fn delete_endpoint(
  State(state): State<AppState>,
  Json(request): Json<AddEndpointRequest>,
) -> StatusCode {
  let mut registry = state.registry.write().await;

  registry.delete(&request.method, &request.path);

  StatusCode::OK
}
