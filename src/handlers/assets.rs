use axum::Json;
use serde::Deserialize;

use crate::{
    auth::admin::Admin,
    error::AppError,
    models::asset::Asset,
    repositories::assets::{AssetRepository, DeleteAssetOutcome},
};

#[tracing::instrument(skip_all)]
pub async fn list_assets(repository: AssetRepository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    pub name: String,
    pub unit_value: f64,
}

#[tracing::instrument(skip_all)]
pub async fn create_asset(
    _: Admin,
    repository: AssetRepository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    pub id: i64,
    pub name: Option<String>,
    pub unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
pub async fn update_asset(
    _: Admin,
    repository: AssetRepository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository
        .update_asset(request.id, request.name, request.unit_value)
        .await?
    {
        Some(update_asset) => Ok(Json(update_asset)),
        None => Err(AppError::AssetDoesNotExist),
    }
}

#[derive(Deserialize)]
pub struct DeleteAssetRequest {
    pub id: i64,
}

#[tracing::instrument(skip_all)]
pub async fn delete_asset(
    _: Admin,
    repository: AssetRepository,
    Json(request): Json<DeleteAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository.delete_asset(request.id).await? {
        DeleteAssetOutcome::Deleted(delete_asset) => Ok(Json(delete_asset)),
        DeleteAssetOutcome::NotFound => Err(AppError::AssetDoesNotExist),
        DeleteAssetOutcome::HasHistory => Err(AppError::AssetCannotBeDeletedBecauseHasHistory),
    }
}

#[cfg(test)]
mod tests {
    use axum::Json;
    use sqlx::PgPool;

    use crate::auth::admin::Admin;
    use crate::error::AppError;
    use crate::handlers::assets::{CreateAssetRequest, DeleteAssetRequest, UpdateAssetRequest};

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };

        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 10.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("success");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");

        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Ethereum".to_string()),
            unit_value: Some(20.0),
        };

        let Json(updated_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(updated_asset.id, 1);
        assert_eq!(updated_asset.name, "Ethereum");
        assert_eq!(updated_asset.unit_value, 20.0);

        insta::assert_json_snapshot!(updated_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_delete_asset(db: PgPool) {
        let request = DeleteAssetRequest { id: 1 };

        let Json(deleted_asset) = delete_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(deleted_asset.id, 1);
        assert_eq!(deleted_asset.name, "Bitcoin");
        assert_eq!(deleted_asset.unit_value, 10.0);

        insta::assert_json_snapshot!(deleted_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset_with_history"))]
    async fn test_delete_asset_with_history(db: PgPool) {
        let request = DeleteAssetRequest { id: 1 };

        let result = delete_asset(Admin, db.into(), Json(request)).await;

        assert!(matches!(
            result,
            Err(AppError::AssetCannotBeDeletedBecauseHasHistory)
        ));
    }
}
