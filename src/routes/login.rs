use axum::{Router, routing::get};
use crate::{
    app::AppState,
    handlers::login::{
        index,
        login,
        login_page,
        logout
    }
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
}