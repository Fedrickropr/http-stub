use axum::{
  Json, Router,
  body::Body,
  extract::State,
  http::{Response, StatusCode, header},
  response::IntoResponse,
  routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

use crate::{persistence, registry::{ResponseBody, ResponseHeader}, state::AppState};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddEndpointRequest {
  method: String,
  path: String,
  response_code: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEndpointRequest {
  method: String,
  path: String,
  response_code: u16,
  body: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
	pub headers: Vec<ResponseHeader>	
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddHeaderRequest {
  key: String,
  value: String,
  method: String,
  path: String
}

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/__stub/endpoint", get(get_endpoint))
    .route("/__stub/endpoint", post(add_endpoint))
    .route("/__stub/endpoint", put(add_endpoint_body))
    .route("/__stub/endpoint", delete(delete_endpoint))
    .route("/__stub/config", get(get_config))
    .route("/__stub/config", put(put_config))
    .route("/__stub/header", put(put_header))
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
			headers: response.headers.clone()
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

async fn get_config(State(state): State<AppState>) -> Response<Body> {
  let registry = state.registry.read().await;

  match persistence::serialize(&registry) {
    Ok(json) => Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "application/json")
      .body(Body::from(json))
      .unwrap(),

    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

async fn put_config(State(state): State<AppState>, body: String) -> StatusCode {
  let registry = match persistence::deserialize(&body) {
    Ok(registry) => registry,
    Err(_) => return StatusCode::BAD_REQUEST,
  };

  *state.registry.write().await = registry;

  StatusCode::NO_CONTENT
}

async fn put_header(
  State(state): State<AppState>,
  Json(request): Json<AddHeaderRequest>
) -> StatusCode {
  let mut registry = state.registry.write().await;

  match registry.add_header(
    &request.method,
    &request.path,
    &request.key,
    &request.value,
  ) {
    Ok(()) => StatusCode::CREATED,
    Err(_) => StatusCode::NOT_FOUND,
  }
}
