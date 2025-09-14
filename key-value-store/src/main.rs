mod application;
mod domain;

use std::sync::Arc;

use application::handlers;
use domain::engine::Engine;
use tokio::signal;
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .expect("setting default subscriber failed");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        handlers::build_router(Arc::new(Engine::new())).into_make_service(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("msg");
    };

    tokio::select! {
        _ = ctrl_c => {}
    }
}
