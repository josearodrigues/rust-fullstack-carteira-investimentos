use axum::{extract::FromRequestParts, http::header::AUTHORIZATION};

use crate::{app::AppState, error::AppError};

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AppError::MissingAuthorization)?
            .to_str()
            .map_err(|_| AppError::InvalidCredentials)?;

        let token = auth
            .strip_prefix("Bearer ")
            .ok_or(AppError::InvalidCredentials)?;

        if token == state.admin_token {
            Ok(Admin)
        } else {
            Err(AppError::InvalidCredentials)
        }
    }
}
