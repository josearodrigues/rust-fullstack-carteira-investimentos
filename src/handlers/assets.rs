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
