use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::PgPool;

use crate::app::AppState;
use crate::models::user::UserRecord;

pub struct UserRepository {
    db: PgPool,
}

impl UserRepository {
    pub async fn add_user(&self, username: &str, password_hash: &str) -> sqlx::Result<UserRecord> {
        sqlx::query_as::<_, UserRecord>(
            "INSERT INTO users (username, password_hash)
             VALUES ($1, $2)
             RETURNING id, username, password_hash;",
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_name(&self, username: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as::<_, UserRecord>(
            "SELECT id, username, password_hash
             FROM users
             WHERE username = $1;",
        )
        .bind(username)
        .fetch_optional(&self.db)
        .await
    }
}

impl FromRequestParts<AppState> for UserRepository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for UserRepository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
