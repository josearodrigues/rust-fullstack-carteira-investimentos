use crate::{
    app::AppState,
    auth::user::User,
    error::AppError,
    models::{
        owned_asset::OwnedAsset,
        portfolio_summary::{PortfolioDistribution, PortfolioHistoryPoint, PortfolioSummary},
    },
    repositories::owned_assets::OwnedAssetRepository,
};
use askama::Template;
use axum::{extract::State, response::Html};
// use serde_json::json;
// use std::collections::BTreeMap;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    pub owned_assets: Vec<OwnedAsset>,
    pub summary: PortfolioSummary,
    pub total_value: f64,
    pub distribution: Vec<PortfolioDistribution>,
    pub history: Vec<PortfolioHistoryPoint>,
    pub distribution_json: String,
    pub history_json: String,
    pub user: User,
}

pub async fn assets(
    State(_state): State<AppState>,
    owned_asset_repository: OwnedAssetRepository,
    user: User,
) -> Result<Html<String>, AppError> {
    // Fetch owned assets for the user
    let owned_assets = owned_asset_repository.list_owned_assets(user.id()).await?;
    // Compute portfolio summary using repository
    let summary = owned_asset_repository
        .fetch_portfolio_summary(user.id())
        .await?;

    let total_value: f64 = owned_assets
        .iter()
        .map(|asset| asset.quantity_owned * asset.unit_value)
        .sum();

    let distribution: Vec<PortfolioDistribution> = owned_assets
        .iter()
        .map(|asset| {
            let value = asset.quantity_owned * asset.unit_value;

            let percentage = if total_value == 0.0 {
                0.0
            } else {
                value / total_value * 100.0
            };

            PortfolioDistribution {
                name: asset.name.clone(),
                value,
                percentage,
            }
        })
        .collect();

    // Aqui começa o backend do histórico
    let mut operations = Vec::new();

    for asset in &owned_assets {
        for transaction in &asset.purchase_history.0 {
            operations.push(transaction);
        }
    }

    operations.sort_by_key(|transaction| transaction.occurred_at);

    let mut accumulated = 0.0;
    let mut history = Vec::new();

    for transaction in operations {
        accumulated += transaction.value_delta;

        history.push(PortfolioHistoryPoint {
            date: transaction
                .occurred_at
                .to_offset(time::UtcOffset::from_hms(-3, 0, 0).unwrap())
                .format(time::macros::format_description!(
                    "[day]/[month]/[year] [hour]:[minute]"
                ))
                .unwrap_or_default(),

            value: accumulated,
        });
    }

    // Converte os dados para o gráfico de distribuição em JSON
    let distribution_json =
        serde_json::to_string(&distribution).expect("failed to serialize portfolio distribution");

    // Converte os dados para o gráfico de histórico em JSON
    let history_json =
        serde_json::to_string(&history).expect("failed to serialize portfolio history");

    let html = DashboardPage {
        owned_assets,
        summary,
        total_value,
        distribution,
        history,
        distribution_json,
        history_json,
        user,
    }
    .render()?;

    Ok(Html(html))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::AppState, auth::user::UnauthenticatedUser,
        models::transaction_history::AssetOperation, repositories::users::UserRepository,
    };
    use sqlx::PgPool;

    fn test_state(db: PgPool) -> AppState {
        AppState {
            db,
            admin_token: "test-token".to_string(),
        }
    }

    #[sqlx::test]
    async fn test_dashboard_renders_summary_and_assets(db: PgPool) {
        let user = UnauthenticatedUser::new("satoshi".to_string(), "password".to_string())
            .register(UserRepository::from(db.clone()))
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO assets (id, name, unit_value)
             VALUES (1, 'Bitcoin', 10.0)",
        )
        .execute(&db)
        .await
        .unwrap();

        let repository = OwnedAssetRepository::from(db.clone());

        repository
            .insert_owned_asset(user.id(), 1, 2.0, 5.0, AssetOperation::Buy)
            .await
            .unwrap();

        let result = assets(State(test_state(db.clone())), repository, user)
            .await
            .unwrap();

        let html = result.0;

        assert!(html.contains("Painel de Investimentos"));
        assert!(html.contains("Bitcoin"));
        assert!(html.contains("Patrimônio"));
        assert!(html.contains("Investido"));
        assert!(html.contains("Rentabilidade"));
        assert!(html.contains("Distribuição da Carteira"));
        assert!(html.contains("Evolução do Patrimônio"));
    }

    #[sqlx::test]
    async fn test_dashboard_distribution_contains_asset_data(db: PgPool) {
        let user = UnauthenticatedUser::new("satoshi".to_string(), "password".to_string())
            .register(UserRepository::from(db.clone()))
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO assets (id, name, unit_value)
             VALUES
                (1, 'Bitcoin', 100.0),
                (2, 'Ethereum', 50.0)",
        )
        .execute(&db)
        .await
        .unwrap();

        let repository = OwnedAssetRepository::from(db.clone());

        repository
            .insert_owned_asset(user.id(), 1, 2.0, 80.0, AssetOperation::Buy)
            .await
            .unwrap();

        repository
            .insert_owned_asset(user.id(), 2, 2.0, 40.0, AssetOperation::Buy)
            .await
            .unwrap();

        let result = assets(State(test_state(db.clone())), repository, user)
            .await
            .unwrap();

        let html = result.0;

        assert!(html.contains("Bitcoin"));
        assert!(html.contains("Ethereum"));

        // Os dados reais são enviados ao JavaScript.
        assert!(html.contains("\"name\":\"Bitcoin\""));
        assert!(html.contains("\"name\":\"Ethereum\""));
    }

    #[sqlx::test]
    async fn test_dashboard_history_contains_operations(db: PgPool) {
        let user = UnauthenticatedUser::new("satoshi".to_string(), "password".to_string())
            .register(UserRepository::from(db.clone()))
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO assets (id, name, unit_value)
             VALUES (1, 'Bitcoin', 100.0)",
        )
        .execute(&db)
        .await
        .unwrap();

        let repository = OwnedAssetRepository::from(db.clone());

        repository
            .insert_owned_asset(user.id(), 1, 2.0, 80.0, AssetOperation::Buy)
            .await
            .unwrap();

        repository
            .insert_owned_asset(user.id(), 1, 1.0, 90.0, AssetOperation::Sell)
            .await
            .unwrap();

        let result = assets(State(test_state(db.clone())), repository, user)
            .await
            .unwrap();

        let html = result.0;

        // O dashboard deve gerar o JSON utilizado pelo gráfico
        // de evolução do patrimônio.
        assert!(html.contains("Evolução do Patrimônio"));
        assert!(html.contains("history"));
        assert!(html.contains("Bitcoin"));
    }
}
