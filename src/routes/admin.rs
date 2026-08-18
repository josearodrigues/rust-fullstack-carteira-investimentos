use axum::{Router, routing::{get, post}};

use crate::{
    app::AppState,
    handlers::admin::{
        create_asset,
        delete_asset,
        list_assets,
        login,
        login_page,
        logout,
        update_asset,
    }
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
        .route("/assets", get(list_assets).post(create_asset))
        .route("/assets/{id}", post(update_asset))
        .route("/assets/{id}/delete", post(delete_asset))
}