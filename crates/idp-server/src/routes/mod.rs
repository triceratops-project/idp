mod index;

use std::sync::Arc;

use axum::{routing::get, Router};

use idp_config::IdpConfig;
use idp_state::IdpState;
use index::IndexHandler;

pub async fn router() -> Router {
    let config = IdpConfig::read().unwrap();
    let state = IdpState::new(&config).await.unwrap();

    let app_state = Arc::new(state);

    Router::new()
        .route("/", get(IndexHandler::get))
        .with_state(app_state)
}
