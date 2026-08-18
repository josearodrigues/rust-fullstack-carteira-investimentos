use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::PgPool;

use crate::app::AppState;
use crate::models::asset::Asset;

pub struct AssetRepository {
    db: PgPool,
}

pub enum DeleteAssetOutcome {
    Deleted(Asset),
    NotFound,
    HasHistory,
}

impl AssetRepository {
    pub async fn list_assets(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as::<_, Asset>(
            "SELECT id, name, unit_value
             FROM assets;",
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn create_asset(&self, name: String, unit_value: f64) -> sqlx::Result<Asset> {
        sqlx::query_as::<_, Asset>(
            "INSERT INTO assets (name, unit_value)
             VALUES ($1, $2)
             RETURNING id, name, unit_value;",
        )
        .bind(name)
        .bind(unit_value)
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        id: i64,
        name: Option<String>,
        unit_value: Option<f64>,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as::<_, Asset>(
            "UPDATE assets
             SET name=COALESCE($2, name),
                 unit_value=COALESCE($3, unit_value)
             WHERE id=$1
             RETURNING id, name, unit_value;",
        )
        .bind(id)
        .bind(name)
        .bind(unit_value)
        .fetch_optional(&self.db)
        .await
    }

    pub async fn delete_asset(&self, id: i64) -> sqlx::Result<DeleteAssetOutcome> {
        let asset = sqlx::query_as::<_, Asset>(
            "SELECT id, name, unit_value
             FROM assets
             WHERE id = $1;",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;

        let Some(asset) = asset else {
            return Ok(DeleteAssetOutcome::NotFound);
        };

        let has_history = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM owned_assets
                WHERE asset_id = $1
            );",
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        if has_history {
            return Ok(DeleteAssetOutcome::HasHistory);
        }

        sqlx::query(
            "DELETE FROM assets
             WHERE id = $1;",
        )
        .bind(id)
        .execute(&self.db)
        .await?;

        Ok(DeleteAssetOutcome::Deleted(asset))
    }
}

impl FromRequestParts<AppState> for AssetRepository {
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
impl From<PgPool> for AssetRepository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
