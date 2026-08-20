use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::PgPool;

use crate::app::AppState;
use crate::models::owned_asset::OwnedAsset;
use crate::models::portfolio_summary::PortfolioSummary;
use crate::models::transaction_history::AssetOperation;

#[derive(Clone)]
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
             SUM(
              CASE
               WHEN o.operation_type = 'BUY'
                THEN (a.unit_value - o.bought_for) * o.quantity_owned
               WHEN o.operation_type = 'SELL'
                THEN -(a.unit_value - o.bought_for) * o.quantity_owned
              END
             ) AS value_delta,
             SUM(
              CASE
               WHEN o.operation_type = 'BUY'
                THEN o.quantity_owned
               WHEN o.operation_type = 'SELL'
                THEN -o.quantity_owned
              END
             ) AS quantity_owned,
             COALESCE(
              JSON_AGG(
               JSON_BUILD_OBJECT(
                'operation_type', o.operation_type,
                'occurred_at', o.timestamp,
                'unit_value', o.bought_for,
                'quantity_bought', o.quantity_owned,
                'value_delta',
                 CASE
                  WHEN o.operation_type = 'BUY'
                   THEN (a.unit_value - o.bought_for) * o.quantity_owned
                  WHEN o.operation_type = 'SELL'
                   THEN -(a.unit_value - o.bought_for) * o.quantity_owned
                END
               )
               ORDER BY o.timestamp DESC
              ),
              '[]'::json
             ) AS purchase_history
            FROM assets AS a
            JOIN owned_assets AS o
              ON o.asset_id = a.id
            WHERE o.user_id = $1
            GROUP BY a.id
            ORDER BY a.name;
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
        operation_type: AssetOperation,
    ) -> sqlx::Result<()> {
        let operation = match operation_type {
            AssetOperation::Buy => "BUY".to_string(),
            AssetOperation::Sell => "SELL".to_string(),
        };
        sqlx::query(
            "INSERT INTO owned_assets
             (user_id, asset_id, quantity_owned, bought_for, operation_type)
             VALUES ($1, $2, $3, $4, $5::asset_operation)",
        )
        .bind(user_id)
        .bind(asset_id)
        .bind(quantity)
        .bind(unit_value)
        .bind(operation)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn fetch_portfolio_summary(&self, user_id: i64) -> sqlx::Result<PortfolioSummary> {
        let row = sqlx::query!(
            r#"
            SELECT
             COALESCE(SUM(
              CASE
               WHEN o.operation_type = 'BUY'  THEN  o.quantity_owned * a.unit_value
               WHEN o.operation_type = 'SELL' THEN -o.quantity_owned * a.unit_value
              END
             ), 0.0) AS "patrimony!: f64",
             COALESCE(SUM(
              CASE
               WHEN o.operation_type = 'BUY'  THEN  o.quantity_owned * o.bought_for
               WHEN o.operation_type = 'SELL' THEN -o.quantity_owned * o.bought_for
              END
             ), 0.0) AS "invested!: f64",
             COUNT(DISTINCT o.asset_id) AS "total_assets!: i64",
             COUNT(*) AS "total_operations!: i64"
            FROM owned_assets AS o
            JOIN assets AS a ON a.id = o.asset_id
            WHERE o.user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(PortfolioSummary::new(
            row.patrimony,
            row.invested,
            row.total_assets,
            row.total_operations,
        ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_owned_assets_buy_and_sell(db: PgPool) {
        sqlx::query(
            "INSERT INTO users (id, username, password_hash) VALUES (1, 'alice', 'hashed')",
        )
        .execute(&db)
        .await
        .unwrap();

        sqlx::query("INSERT INTO assets (id, name, unit_value) VALUES (10, 'TestAsset', 100.0)")
            .execute(&db)
            .await
            .unwrap();

        let repo = OwnedAssetRepository::from(db.clone());

        repo.insert_owned_asset(1, 10, 10.0, 80.0, AssetOperation::Buy)
            .await
            .unwrap();

        let owned = repo.list_owned_assets(1).await.unwrap();
        assert_eq!(owned.len(), 1);
        assert_eq!(owned[0].quantity_owned, 10.0);
        assert_eq!(owned[0].value_delta, 200.0);
        assert_eq!(owned[0].purchase_history.0.len(), 1);
        assert_eq!(
            owned[0].purchase_history.0[0].operation_type,
            AssetOperation::Buy
        );
        assert_eq!(owned[0].purchase_history.0[0].quantity_bought, 10.0);
        assert_eq!(owned[0].purchase_history.0[0].unit_value, 80.0);
        assert_eq!(owned[0].purchase_history.0[0].value_delta, 200.0);

        repo.insert_owned_asset(1, 10, 4.0, 90.0, AssetOperation::Sell)
            .await
            .unwrap();

        let owned = repo.list_owned_assets(1).await.unwrap();
        assert_eq!(owned.len(), 1);
        assert_eq!(owned[0].quantity_owned, 6.0);
        assert_eq!(owned[0].value_delta, 160.0);
        assert_eq!(owned[0].purchase_history.0.len(), 2);

        let tx1 = &owned[0].purchase_history.0[1];
        let tx2 = &owned[0].purchase_history.0[0];

        assert_eq!(tx1.operation_type, AssetOperation::Buy);
        assert_eq!(tx1.value_delta, 200.0);

        assert_eq!(tx2.operation_type, AssetOperation::Sell);
        assert_eq!(tx2.value_delta, -40.0);
    }
}
