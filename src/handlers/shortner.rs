use axum::{
    Json,
    extract::{Path, State},
    response::Redirect,
};

use crate::{
    db::{shortner::ShortnerRepo, user::UserRepo},
    error::AppResult,
    models::shortner::{Link, UploadLink},
};

#[derive(Clone)]
pub struct AppState {
    pub shortner_repo: ShortnerRepo,
    pub user_repo: UserRepo,
}

pub async fn create_link_handler(
    State(state): State<AppState>,
    Json(payload): Json<UploadLink>,
) -> AppResult<Json<Link>> {
    let link = state.shortner_repo.create_link(payload.url).await?;
    Ok(Json(link))
}

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Redirect> {
    let link = state.shortner_repo.find_by_slug(slug).await?;
    Ok(Redirect::to(&link.url))
}
