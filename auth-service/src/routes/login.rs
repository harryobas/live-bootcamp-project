
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::{error::AuthAPIError, user::User, Email, Password},
    utils::auth::generate_auth_cookie
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>
) ->  Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email =  Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let password =  Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.clone();

   user_store.validate_user(email.as_ref(), password.as_ref())
    .await
    .map_err(|_| AuthAPIError::InCorrectCredentials)?;
    

   //if let Err(_) = user_store.get_user(email.as_ref()).await {
    //return (jar, Err(AuthAPIError::InCorrectCredentials))
   //}

   let auth_cookie =  generate_auth_cookie(&email)
        .map_err(|_| AuthAPIError::UnexpectedError)?; 

   let updated_jar = jar.add(auth_cookie);

   Ok((updated_jar, StatusCode::OK))
    
}

#[derive(Deserialize)]
pub struct LoginRequest{
    email: String,
    password: String

}

