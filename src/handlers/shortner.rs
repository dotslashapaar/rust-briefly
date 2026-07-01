use axum::{
    Json,
    extract::{Path, State},
    response::Redirect,
};

use crate::{
    db::{shortner::ShortnerRepo, user::UserRepo},
    error::AppResult,
    middleware::auth::AuthUser,
    models::shortner::{Link, UploadLink},
};

#[derive(Clone)]
pub struct AppState {
    pub shortner_repo: ShortnerRepo,
    pub user_repo: UserRepo,
    pub jwt_secret: String,
}

pub async fn create_link_handler(
    State(state): State<AppState>,
    user: Option<AuthUser>,
    Json(payload): Json<UploadLink>,
) -> AppResult<Json<Link>> {
    let account = user.map(|AuthUser(id)| id);
    let link = state
        .shortner_repo
        .create_link(payload.url, account)
        .await?;
    Ok(Json(link))
}

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Redirect> {
    let link = state.shortner_repo.find_by_slug(slug).await?;
    Ok(Redirect::to(&link.url))
}

pub async fn list_my_links(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Link>>> {
    let links = state.shortner_repo.list_by_account(user_id).await?;
    Ok(Json(links))
}
