use askama::Template;
use axum::{extract::{Path, State}, Form, response::{Html, IntoResponse, Redirect}};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    app::AppState,
    auth::admin::Admin,
    error::AppError,
    models::asset::Asset,
    repositories::assets::{AssetRepository, DeleteAssetOutcome},
};

#[derive(Template)]
#[template(path = "admin_login.html")]
struct AdminLoginPage;

#[tracing::instrument(skip_all)]
pub async fn login_page() -> Result<Html<String>, AppError> {
    let html = AdminLoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct AdminLoginForm {
    admin_token: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(request): Form<AdminLoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let token = request.admin_token;

    if token != state.admin_token {
        return Err(AppError::InvalidCredentials);
    }

    let cookie = Cookie::build(("admin_token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/admin/assets")))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove("admin_token"), Redirect::to("/admin/login"))
}

#[derive(Template)]
#[template(path = "admin_assets.html")]
pub struct AdminAssetsPage {
    assets: Vec<Asset>,
}

pub async fn list_assets(
    _: Admin,
    repository: AssetRepository,
) -> Result<Html<String>, AppError> {
    let assets = repository.list_assets().await?;

    let html = AdminAssetsPage { assets }.render()?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct CreateAssetForm {
    pub name: String,
    pub unit_value: f64,
}

#[tracing::instrument(skip_all)]
pub async fn create_asset(
    _: Admin,
    repository: AssetRepository,
    Form(request): Form<CreateAssetForm>,
) -> Result<Redirect, AppError> {
    repository
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Redirect::to("/admin/assets"))
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    pub name: Option<String>,
    pub unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
pub async fn update_asset(
    _: Admin,
    repository: AssetRepository,
    Path(id): Path<i64>,
    Form(request): Form<UpdateAssetRequest>,
) -> Result<Redirect, AppError> {
    match repository
        .update_asset(id, request.name, request.unit_value)
        .await?
    {
        Some(_) => Ok(Redirect::to("/admin/assets")),
        None => Err(AppError::AssetDoesNotExist),
    }
}

#[tracing::instrument(skip_all)]
pub async fn delete_asset(
    _: Admin,
    repository: AssetRepository,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    match repository.delete_asset(id).await? {
        DeleteAssetOutcome::Deleted(_delete_asset) => Ok(Redirect::to("/admin/assets")),
        DeleteAssetOutcome::NotFound => Err(AppError::AssetDoesNotExist),
        DeleteAssetOutcome::HasHistory => Err(AppError::AssetCannotBeDeletedBecauseHasHistory),
    }
}
