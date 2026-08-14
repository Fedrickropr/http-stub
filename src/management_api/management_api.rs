use axum::{Json, Router, extract::State, http::StatusCode, routing::{delete, post}};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
struct AddEndpointRequest {
  method: String,
  path: String,
  response_code: u16,
}

pub fn router() -> Router<AppState> {
  Router::new().route("/__stub/endpoints", post(add_endpoint)).route("/__stub/endpoints", delete(delete_endpoint))
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
