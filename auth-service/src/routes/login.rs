
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{error::AuthAPIError, user::User, Email, Password}};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let password = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.clone();

   user_store.validate_user(email.as_ref(), password.as_ref()).await
        .map_err(|_| AuthAPIError::InCorrectCredentials)?;

    let _user = user_store.get_user(email.as_ref()).await
       .map_err(|_| AuthAPIError::InCorrectCredentials)?;

    Ok(StatusCode::OK.into_response())
    
}

#[derive(Deserialize)]
pub struct LoginRequest{
    email: String,
    password: String

}

