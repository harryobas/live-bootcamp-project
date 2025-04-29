use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::error::AuthAPIError, 
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(
    jar: CookieJar,
    State(state): State<AppState>
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    // Get JWT cookie or return 400
    let cookie = jar.get(JWT_COOKIE_NAME)
        .ok_or(AuthAPIError::MissingToken)?
        .clone();

    let token = cookie.value();

    // Validate token before banning
    validate_token(token, state.banned_tokens_store.clone())
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    // Add token to banned list
    state.banned_tokens_store
        .write()
        .await
        .add_token(token)
        .await;

    // Remove cookie from jar
    let updated_jar = jar.remove(cookie);

    Ok((updated_jar, StatusCode::OK))
}


