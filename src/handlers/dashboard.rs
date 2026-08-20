use crate::{
    app::AppState,
    auth::user::User,
    error::AppError,
    models::{
        owned_asset::OwnedAsset,
        portfolio_summary::{
            PortfolioSummary,
            PortfolioDistribution,
            PortfolioHistoryPoint,
        },
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
                .format(
                    time::macros::format_description!(
                        "[day]/[month]/[year] [hour]:[minute]"
                    )
                )
                .unwrap_or_default(),

            value: accumulated,
        });
    }

    // Converte os dados para o gráfico de distribuição em JSON
    let distribution_json =
        serde_json::to_string(&distribution)
        .expect("failed to serialize portfolio distribution");

    // Converte os dados para o gráfico de histórico em JSON
    let history_json =
        serde_json::to_string(&history)
        .expect("failed to serialize portfolio history");

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
