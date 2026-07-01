use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::{
    error::{AppError, AppResult},
    handlers::shortner::AppState,
    models::shortner::{UserLogin, UserRegister},
    utils::auth::{encode_jwt, hash_password, verify_password},
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<UserRegister>,
) -> AppResult<Json<Value>> {
    let password_hash = hash_password(&payload.password)?;

    let user = state
        .user_repo
        .create_user(payload.username, password_hash)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(sqlx::Error::Database(db)) if db.is_unique_violation() => {
                AppError::Conflict("username already taken".into())
            }
            other => other,
        })?;

    let token = encode_jwt(user.id, &state.jwt_secret)?;

    Ok(Json(json!({"token": token})))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<UserLogin>,
) -> AppResult<Json<Value>> {
    let user = state
        .user_repo
        .find_by_username(payload.username)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = encode_jwt(user.id, &state.jwt_secret)?;
    Ok(Json(json!({"token": token})))
}
