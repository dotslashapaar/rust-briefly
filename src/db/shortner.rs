use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::shortner::Link,
    utils::url_shortner,
};

#[derive(Clone)]
pub struct ShortnerRepo {
    pool: PgPool,
}

impl ShortnerRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_link(&self, url: String, account: Option<Uuid>) -> AppResult<Link> {
        let short_url = url_shortner(&url);
        let id = Uuid::new_v4();

        let link = sqlx::query_as!(
            Link,
            "INSERT INTO links (id, url, short_url, account) VALUES ($1, $2, $3, $4) \
             ON CONFLICT (short_url) DO UPDATE SET short_url = links.short_url \
             RETURNING *",
            id,
            url,
            short_url,
            account
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(link)
    }

    pub async fn find_by_slug(&self, short_url: String) -> AppResult<Link> {
        let link = sqlx::query_as!(Link, "SELECT * FROM links WHERE short_url = $1", short_url)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("url not found")))?;

        Ok(link)
    }

    pub async fn list_by_account(&self, account: Uuid) -> AppResult<Vec<Link>> {
        let links = sqlx::query_as!(Link, "SELECT * FROM links WHERE account = $1", account)
            .fetch_all(&self.pool)
            .await?;

        Ok(links)
    }
}
