use axum::Router;

use crate::app::AppState;
use crate::routes::assets;

pub fn router() -> Router<AppState> {
    Router::new().merge(assets::router())
}
