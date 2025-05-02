use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{data_stores::{LoginAttemptId, TwoFACode}, 
    error::AuthAPIError, Email}, 
    utils::auth::generate_auth_cookie
};

pub async  fn verify_2fa(
    State(state): State<AppState>,
    jar:  CookieJar,
    Json(request): Json<Verify2FARequest>  
) -> Result<impl IntoResponse, AuthAPIError>{
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let login_attempt_id = LoginAttemptId::parse(
        &request.login_attempt_id
    ).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let two_fa_code = TwoFACode::parse(
        &request.two_fa_code
    ).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let two_fa_code_store = state.two_fa_code_store;

    let (id, code) = two_fa_code_store.read().await.get_code(&email)
        .await
        .map_err(|_| AuthAPIError::InCorrectCredentials)?;

    if !(id.as_ref() == login_attempt_id.as_ref() && code.as_ref() == two_fa_code.as_ref() ) {
        return Err(AuthAPIError::InCorrectCredentials)
    }

    two_fa_code_store
        .write()
        .await
        .remove_code(&email)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    generate_auth_cookie(&email)
        .map(|cookie| {
            let updated_jar = jar.add(cookie);
            (updated_jar, StatusCode::OK)
        })
        .map_err(|_| AuthAPIError::UnexpectedError)

}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String
}