use axum::{Router, routing::get};

use crate::app::AppState;
use crate::handlers::dashboard::assets;

pub fn router() -> Router<AppState> {
    Router::new().route("/dashboard", get(assets))
}
