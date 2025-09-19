use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
};

use tower::ServiceBuilder;
use tower_http::{
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{application::errors::AppError, domain::{engine::Manager}};

#[derive(Clone)]
pub struct AppState {
    pub manager: Arc<dyn Manager + Send + Sync>,
}

pub fn build_router(manager: Arc<dyn Manager + Send + Sync + 'static>) -> Router {
    let state = AppState { manager };

    let app = Router::new()
        .route("/get/{id}", get(get_handler))
        .route("/set/{id}", post(set_handler))
        .route("/delete/{id}", delete(delete_handler))
        .route("/total", get(total_handler))
        .route("/clear", post(clear_handler))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO))
                )
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
        )
        .with_state(state);

    app
}

async fn get_handler(
    Path(id): Path<String>,
    State(AppState { manager }): State<AppState>,
) -> Result<Json<String>, AppError> {
    let value = manager.get(&id)?;
    Ok(Json(value))
}

async fn set_handler(
    Path(id): Path<String>,
    State(AppState { manager }): State<AppState>,
    body: String,
) -> Result<StatusCode, AppError> {
    manager.set(&id, &body)?;
    Ok(StatusCode::OK)
}

async fn delete_handler(
    Path(id): Path<String>,
    State(AppState { manager }): State<AppState>,
) -> Result<Json<String>, AppError> {
    let value = manager.delete(&id)?;
    Ok(Json(value))
}

async fn total_handler(
    State(AppState { manager }): State<AppState>,
) -> Result<Json<usize>, AppError> {
    let size = manager.total_size()?;
    Ok(Json(size))
}

async fn clear_handler(
    State(AppState { manager }): State<AppState>,
) -> Result<StatusCode, AppError> {
    manager.clear()?;
    Ok(StatusCode::OK)
}
