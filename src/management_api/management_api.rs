use axum::{
  Json, Router,
  extract::State,
  http::StatusCode,
  routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

use crate::{registry::ResponseBody, state::AppState};

#[derive(Deserialize)]
struct AddEndpointRequest {
  method: String,
  path: String,
  response_code: u16,
}

#[derive(Deserialize)]
struct UpdateEndpointRequest {
  method: String,
  path: String,
  response_code: u16,
  body: String,
}

#[derive(Deserialize)]
struct DeleteEndpointRequest {
  method: String,
  path: String,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
  pub path: String,
  pub method: String,
  pub response_code: u16,
  pub body: Option<ResponseBody>,
}

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/__stub/endpoint", get(get_endpoint))
    .route("/__stub/endpoint", post(add_endpoint))
    .route("/__stub/endpoint", put(add_endpoint_body))
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
      body: response.body.clone(),
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

async fn add_endpoint_body(
  State(state): State<AppState>,
  Json(request): Json<UpdateEndpointRequest>,
) -> StatusCode {
  let mut registry = state.registry.write().await;

  registry.add_with_string_body(
    &request.method,
    &request.path,
    request.response_code,
    &request.body,
  );

  StatusCode::CREATED
}

async fn delete_endpoint(
  State(state): State<AppState>,
  Json(request): Json<DeleteEndpointRequest>,
) -> StatusCode {
  let mut registry = state.registry.write().await;

  registry.delete(&request.method, &request.path);

  StatusCode::OK
}
