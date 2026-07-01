use axum::{Router, routing::post};

use crate::handlers::{
    auth::{login, register},
    shortner::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
