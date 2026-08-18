use askama::Template;
use axum::{
    Form,
    response::{Html, Redirect},
};
use serde::Deserialize;
use tokio::try_join;

use crate::{
    auth::user::User,
    error::AppError,
    models::asset::Asset,
    models::owned_asset::OwnedAsset,
    models::transaction_history::AssetOperation,
    repositories::assets::AssetRepository,
    repositories::owned_assets::OwnedAssetRepository,
};

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    user: User,
}

pub async fn assets(
    owned_asset_repository: OwnedAssetRepository,
    asset_repository: AssetRepository,
    user: User,
) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        owned_asset_repository.list_owned_assets(user.id()),
        asset_repository.list_assets()
    )?;

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    asset_id: i64,
    unit_value: f64,
    quantity: f64,
    operation_type: AssetOperation,
}

pub async fn purchase_asset(
    repository: OwnedAssetRepository,
    user: User,
    Form(request): Form<PurchaseAssetForm>,
) -> Result<Redirect, AppError> {
    if request.operation_type == AssetOperation::Sell {
        let owned_assets = repository.list_owned_assets(user.id()).await?;
        let owned_quantity = owned_assets
            .iter()
            .find(|asset| asset.id == request.asset_id)
            .map(|asset| asset.quantity_owned)
            .unwrap_or(0.0);

        if owned_quantity < request.quantity {
            return Err(AppError::InsufficientQuantity);
        }
    }

    repository
        .insert_owned_asset(
            user.id(),
            request.asset_id,
            request.quantity,
            request.unit_value,
            request.operation_type,
        )
        .await?;

    Ok(Redirect::to("/assets"))
}

pub mod filters {
    use askama;
    use time::{
        OffsetDateTime, format_description::StaticFormatDescription, macros::format_description,
    };

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        const HUMAN_READABLE_FORMAT: StaticFormatDescription =
            format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]");

        datetime
            .format(HUMAN_READABLE_FORMAT)
            .map_err(askama::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    use crate::auth::user::UnauthenticatedUser;
    use crate::repositories::users::UserRepository;

    #[sqlx::test]
    async fn test_purchase_asset_sell_insufficient_quantity(db: PgPool) {
        let user_repo = UserRepository::from(db.clone());
        let unauth = UnauthenticatedUser::new("bob".to_string(), "pass123".to_string());
        let user = unauth.register(user_repo).await.unwrap();

        sqlx::query("INSERT INTO assets (id, name, unit_value) VALUES (1, 'Bitcoin', 50000.0)")
            .execute(&db)
            .await
            .unwrap();

        let owned_repo = OwnedAssetRepository::from(db.clone());

        let form = PurchaseAssetForm {
            asset_id: 1,
            unit_value: 50000.0,
            quantity: 1.0,
            operation_type: AssetOperation::Sell,
        };

        let result = purchase_asset(owned_repo.clone(), user.clone(), Form(form)).await;
        assert!(matches!(result, Err(AppError::InsufficientQuantity)));

        let form_buy = PurchaseAssetForm {
            asset_id: 1,
            unit_value: 50000.0,
            quantity: 0.5,
            operation_type: AssetOperation::Buy,
        };
        let buy_res = purchase_asset(owned_repo.clone(), user.clone(), Form(form_buy)).await;
        assert!(buy_res.is_ok());

        let form_sell_too_much = PurchaseAssetForm {
            asset_id: 1,
            unit_value: 51000.0,
            quantity: 0.6,
            operation_type: AssetOperation::Sell,
        };
        let sell_res1 = purchase_asset(owned_repo.clone(), user.clone(), Form(form_sell_too_much)).await;
        assert!(matches!(sell_res1, Err(AppError::InsufficientQuantity)));

        let form_sell_ok = PurchaseAssetForm {
            asset_id: 1,
            unit_value: 52000.0,
            quantity: 0.3,
            operation_type: AssetOperation::Sell,
        };
        let sell_res2 = purchase_asset(owned_repo.clone(), user.clone(), Form(form_sell_ok)).await;
        assert!(sell_res2.is_ok());

        let owned = owned_repo.list_owned_assets(user.id()).await.unwrap();
        assert_eq!(owned.len(), 1);
        assert_eq!(owned[0].quantity_owned, 0.2);
    }
}
