use askama::Template;
use axum::{
    Form,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    auth::user::{UnauthenticatedUser, User},
    error::AppError,
    repositories::users::UserRepository,
};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

#[tracing::instrument(skip_all)]
pub async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login(
    repository: UserRepository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(repository).await?,
        Err(other_err) => return Err(other_err),
    };

    let token = user.auth_token()?;

    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove("token"), Redirect::to("/login"))
}

pub async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login")),
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        http::header::{LOCATION, SET_COOKIE},
        response::IntoResponse,
    };
    use axum_extra::extract::cookie::CookieJar;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn login_page_renders(_pool: PgPool) -> Result<(), sqlx::Error> {
        let result: Result<Html<String>, AppError> = login_page().await;
        assert!(result.is_ok());
        let html = result.unwrap();

        assert!(html.0.contains("<form"));

        Ok(())
    }

    #[sqlx::test]
    async fn login_successful_existing_user(pool: PgPool) -> Result<(), sqlx::Error> {
        let repo = UserRepository::from(pool.clone());
        let username = "alice";
        let password = "secret123";

        UnauthenticatedUser::new(username.into(), password.into())
            .register(UserRepository::from(pool.clone()))
            .await
            .expect("registration should succeed");

        let form = Form(LoginForm {
            username: username.into(),
            password: password.into(),
        });
        let jar = CookieJar::new();
        let result = login(repo, jar, form).await;

        assert!(result.is_ok());

        let response = result.unwrap().into_response();
        let location = response
            .headers()
            .get(LOCATION)
            .expect("location header should exist")
            .to_str()
            .unwrap();

        assert_eq!(location, "/");

        let set_cookie = response
            .headers()
            .get(SET_COOKIE)
            .expect("token cookie should be set")
            .to_str()
            .unwrap();

        assert!(set_cookie.starts_with("token="));

        Ok(())
    }

    #[sqlx::test]
    async fn login_auto_register_when_user_missing(pool: PgPool) -> Result<(), sqlx::Error> {
        let repo = UserRepository::from(pool.clone());
        let username = "bob";
        let password = "newpass";
        let form = Form(LoginForm {
            username: username.into(),
            password: password.into(),
        });
        let jar = CookieJar::new();
        let result = login(repo, jar, form).await;

        assert!(result.is_ok());

        let response = result.unwrap().into_response();
        let location = response
            .headers()
            .get(LOCATION)
            .expect("location header should exist")
            .to_str()
            .unwrap();

        assert_eq!(location, "/");

        let set_cookie = response
            .headers()
            .get(SET_COOKIE)
            .expect("token cookie should be set")
            .to_str()
            .unwrap();

        assert!(set_cookie.starts_with("token="));

        Ok(())
    }

    #[sqlx::test]
    async fn login_fails_invalid_credentials(pool: PgPool) -> Result<(), sqlx::Error> {
        let repo = UserRepository::from(pool.clone());
        let username = "carol";
        let password = "correct";

        UnauthenticatedUser::new(username.into(), password.into())
            .register(UserRepository::from(pool.clone()))
            .await
            .expect("registration should succeed");

        let wrong_form = Form(LoginForm {
            username: username.into(),
            password: "wrongpass".into(),
        });
        let jar = CookieJar::new();
        let result = login(repo, jar, wrong_form).await;
        match result {
            Err(AppError::InvalidCredentials) => (),
            _ => panic!("expected InvalidCredentials error"),
        }

        Ok(())
    }

    #[sqlx::test]
    async fn logout_clears_cookie(_pool: PgPool) -> Result<(), sqlx::Error> {
        let jar = CookieJar::new()
            .add(Cookie::build(("token", "dummy")).http_only(true));
        let response = logout(jar).await.into_response();
        // Verify redirect location only
        let location = response.headers().get(LOCATION).unwrap().to_str().unwrap();

        assert_eq!(location, "/login");

        Ok(())
    }

    #[sqlx::test]
    async fn index_redirects_based_on_authentication(pool: PgPool) -> Result<(), sqlx::Error> {
        let _repo = UserRepository::from(pool.clone());
        let user = UnauthenticatedUser::new("dave".into(), "pwd".into())
            .register(UserRepository::from(pool.clone()))
            .await
            .expect("registration should succeed");

        // Authenticated user should redirect to assets
        let resp_auth = index(Some(user.clone()))
            .await
            .expect("authenticated index should succeed")
            .into_response();
        let location_auth = resp_auth.headers().get(LOCATION).unwrap().to_str().unwrap();

        assert_eq!(location_auth, "/assets");

        // No user should redirect to login
        let resp_none = index(None)
            .await
            .expect("unauthenticated index should succeed")
            .into_response();
        let location_none = resp_none.headers().get(LOCATION).unwrap().to_str().unwrap();

        assert_eq!(location_none, "/login");

        Ok(())
    }
}
