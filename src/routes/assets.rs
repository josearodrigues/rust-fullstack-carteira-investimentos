use axum::{Router, routing::get};

use crate::app::AppState;
use crate::handlers::assets::{create_asset, delete_asset, list_assets, update_asset};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets)
            .post(create_asset)
            .patch(update_asset)
            .delete(delete_asset),
    )
}
