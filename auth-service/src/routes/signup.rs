use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{user::User, error::AuthAPIError}};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    if !request.email.contains('@') || request.email.is_empty() || request.password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }
    let user = User::new(
        request.email.clone(),
        request.password.clone(),
        request.requires_2fa
    );

    let mut user_store = state.user_store.write().await;
    if user_store.users.contains_key(&request.email) {
        return Err(AuthAPIError::UserAlreadyExists);
    }

   if let Err(_) = user_store.add_user(user) {
    return Err(AuthAPIError::UnexpectedError);
   }else{
    let response = Json(
        SignupResponse{message: "User created successfully".to_string()}
    );

    Ok((StatusCode::CREATED, response))
}

    
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,

    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String
}