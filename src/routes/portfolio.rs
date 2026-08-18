use axum::{Router, routing::get};

use crate::{
    app::AppState,
    handlers::portfolio::{assets, purchase_asset},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(assets).post(purchase_asset))
}