use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{error::AuthAPIError, user::User, Email, Password}};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    if Email::parse(&request.email).is_err() || Password::parse(&request.password).is_err() {
        return Err(AuthAPIError::InvalidCredentials);
    }
    let email = Email(request.email.clone());
    let password = Password(request.password.clone());

    let user = User::new(
        email,
        password,
        request.requires_2fa
    );

    let  user_store = state.user_store.clone();
    if user_store.get_user(&request.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

   if let Err(_) = user_store.add_user(user).await {
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