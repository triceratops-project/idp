mod index;

use std::sync::Arc;

use axum::{routing::get, Router};

use idp_config::IdpConfig;
use idp_state::IdpState;
use index::StatusHandler;

pub async fn router() -> Router {
    let config = IdpConfig::read().unwrap();
    let state = IdpState::new(&config).await.unwrap();

    let app_state = Arc::new(state);

    Router::new()
        .route("/api/status", get(StatusHandler::get))
        .with_state(app_state)
}
