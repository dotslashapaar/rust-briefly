use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Link {
    pub id: Uuid,
    pub url: String,
    pub short_url: String,
    pub account: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserRegister {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UploadLink {
    pub url: String,
}
