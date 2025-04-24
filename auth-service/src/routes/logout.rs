use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::error::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME)
        .ok_or(AuthAPIError::MissingToken)?
        .clone();

    let token = cookie.value();

    validate_token(token).await.map_err(|_| AuthAPIError::InvalidToken)?;
    
    let jar = jar.remove(cookie);

    Ok((jar, StatusCode::OK))
}

