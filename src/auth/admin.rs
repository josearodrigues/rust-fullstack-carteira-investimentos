use axum::extract::FromRequestParts;
use axum_extra::extract::cookie::CookieJar;

use crate::{app::AppState, error::AppError};

pub struct Admin;

const ADMIN_COOKIE_NAME: &str = "admin_token";

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get(ADMIN_COOKIE_NAME)
            .ok_or(AppError::MissingAuthorization)?
            .value();

        if token == state.admin_token {
            Ok(Admin)
        } else {
            Err(AppError::InvalidCredentials)
        }
    }
}
