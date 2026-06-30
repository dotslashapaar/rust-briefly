use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::shortner::User,
};

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, username: String, password_hash: String) -> AppResult<User> {
        let id = Uuid::new_v4();
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, password_hash, username) VALUES ($1, $2, $3) RETURNING *",
            id,
            password_hash,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: String) -> AppResult<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Username {username} not found")))?;

        Ok(user)
    }
}
