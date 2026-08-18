use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::PgPool;

use crate::app::AppState;
use crate::models::owned_asset::OwnedAsset;

pub struct OwnedAssetRepository {
    db: PgPool,
}

impl OwnedAssetRepository {
    pub async fn list_owned_assets(&self, user_id: i64) -> sqlx::Result<Vec<OwnedAsset>> {
        sqlx::query_as::<_, OwnedAsset>(
            r#"
            SELECT
             a.id,
             a.name,
             a.unit_value,
             SUM((a.unit_value - o.bought_for) * o.quantity_owned) AS value_delta,
             SUM(o.quantity_owned) AS quantity_owned,
             COALESCE(
              JSON_AGG(
               JSON_BUILD_OBJECT(
                'bought_at', o.timestamp,
                'bought_for', o.bought_for,
                'quantity_bought', o.quantity_owned,
                'value_delta', (a.unit_value - o.bought_for) * o.quantity_owned
               )
              ),
              '[]'::json
             ) AS purchase_history
            FROM assets AS a
            JOIN owned_assets AS o
              ON o.asset_id = a.id
            WHERE o.user_id = $1
            GROUP BY a.id;
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: f64,
        unit_value: f64,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO owned_assets
             (user_id, asset_id, quantity_owned, bought_for)
             VALUES ($1, $2, $3, $4);",
        )
        .bind(user_id)
        .bind(asset_id)
        .bind(quantity)
        .bind(unit_value)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

impl FromRequestParts<AppState> for OwnedAssetRepository {
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
impl From<PgPool> for OwnedAssetRepository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
