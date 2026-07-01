use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{error::AppError, handlers::shortner::AppState, utils::auth::decode_jwt};

pub struct AuthUser(pub Uuid);

fn extract_user(parts: &Parts, state: &AppState) -> Result<Option<Uuid>, AppError> {
    let Some(header) = parts.headers.get(AUTHORIZATION) else {
        return Ok(None);
    };

    let header = header.to_str().map_err(|_| AppError::Unauthorized)?;
    let token = header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;
    let claims = decode_jwt(token, &state.jwt_secret)?;
    Ok(Some(claims.sub))
}

impl OptionalFromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(extract_user(parts, state)?.map(AuthUser))
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        extract_user(parts, state)?
            .map(AuthUser)
            .ok_or(AppError::Unauthorized)
    }
}
