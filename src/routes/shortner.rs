use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::shortner::{AppState, create_link_handler, redirect_handler};

pub fn shortner_routes() -> Router<AppState> {
    Router::new()
        .route("/shorten", post(create_link_handler))
        .route("/{slug}", get(redirect_handler))
}
