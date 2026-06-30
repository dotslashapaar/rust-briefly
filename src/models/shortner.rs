use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    #[serde(skip_serializing)]
    password_hash: String,
    username: String,
    created_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Link {
    id: Uuid,
    url: String,
    short_url: String,
    account: Option<Uuid>,
    created_at: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct UserRegister {
    password: String,
    username: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    password: String,
    username: String,
}

#[derive(Deserialize)]
pub struct UploadLink {
    url: String
}