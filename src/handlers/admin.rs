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

#[cfg(test)]
mod tests {
    use axum::{
        extract::Path,
        http::{
            header::{LOCATION, SET_COOKIE},
            StatusCode,
        },
        response::IntoResponse,
        Form,
    };
    use axum_extra::extract::cookie::CookieJar;
    use sqlx::PgPool;

    use crate::{
        app::AppState,
        auth::admin::Admin,
        error::AppError,
        repositories::assets::AssetRepository,
    };

    use super::*;

    fn db_state(db: PgPool, admin_token: &str) -> AppState {
        AppState {
            db,
            admin_token: admin_token.to_string(),
        }
    }

    fn lazy_state(admin_token: &str) -> AppState {
        AppState {
            db: PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
                .expect("lazy pool"),
            admin_token: admin_token.to_string(),
        }
    }

    fn assert_redirect_to(response: &axum::response::Response, location: &str) {
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(response.headers().get(LOCATION).unwrap(), location);
    }

    #[tokio::test]
    async fn test_login_page_renders() {
        let Html(html) = login_page().await.expect("success");

        assert!(html.contains("admin access"));
        assert!(html.contains("admin_token"));
    }

    #[tokio::test]
    async fn test_login_success_sets_admin_cookie() {
        let state = lazy_state("super-secret");
        let request = AdminLoginForm {
            admin_token: "super-secret".to_string(),
        };

        let response = login(State(state), CookieJar::new(), Form(request))
            .await
            .expect("success")
            .into_response();

        assert_redirect_to(&response, "/admin/assets");
        let set_cookie = response.headers().get(SET_COOKIE).unwrap().to_str().unwrap();
        assert!(set_cookie.contains("admin_token=super-secret"));
        assert!(set_cookie.contains("HttpOnly"));
    }

    #[tokio::test]
    async fn test_login_rejects_invalid_token() {
        let state = lazy_state("super-secret");
        let request = AdminLoginForm {
            admin_token: "wrong-token".to_string(),
        };

        let result = login(State(state), CookieJar::new(), Form(request)).await;

        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets_renders_admin_page(db: PgPool) {
        let Html(html) = list_assets(Admin, db.into()).await.expect("success");

        assert!(html.contains("asset management"));
        assert!(html.contains("Bitcoin"));
    }

    #[sqlx::test]
    async fn test_create_asset_redirects_and_persists(db: PgPool) {
        let request = CreateAssetForm {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };

        let response = create_asset(Admin, db.clone().into(), Form(request))
            .await
            .expect("success")
            .into_response();

        assert_redirect_to(&response, "/admin/assets");

        let repository: AssetRepository = db.into();
        let assets = repository.list_assets().await.expect("assets");
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset_redirects_and_updates(db: PgPool) {
        let request = UpdateAssetRequest {
            name: Some("Ethereum".to_string()),
            unit_value: Some(20.0),
        };

        let response = update_asset(Admin, db.clone().into(), Path(1), Form(request))
            .await
            .expect("success")
            .into_response();

        assert_redirect_to(&response, "/admin/assets");

        let repository: AssetRepository = db.into();
        let assets = repository.list_assets().await.expect("assets");
        assert_eq!(assets[0].name, "Ethereum");
        assert_eq!(assets[0].unit_value, 20.0);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_delete_asset_redirects_and_removes(db: PgPool) {
        let response = delete_asset(Admin, db.clone().into(), Path(1))
            .await
            .expect("success")
            .into_response();

        assert_redirect_to(&response, "/admin/assets");

        let repository: AssetRepository = db.into();
        let assets = repository.list_assets().await.expect("assets");
        assert!(assets.is_empty());
    }

    #[sqlx::test(fixtures("bitcoin_asset_with_history"))]
    async fn test_delete_asset_with_history_rejects(db: PgPool) {
        let result = delete_asset(Admin, db.into(), Path(1)).await;

        assert!(matches!(
            result,
            Err(AppError::AssetCannotBeDeletedBecauseHasHistory)
        ));
    }
}
