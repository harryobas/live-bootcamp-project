
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
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(&request.email){
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(&request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.clone();

   if user_store.validate_user(email.as_ref(), password.as_ref()).await.is_err(){
    return (jar, Err(AuthAPIError::InCorrectCredentials))
   };

   //if let Err(_) = user_store.get_user(email.as_ref()).await {
    //return (jar, Err(AuthAPIError::InCorrectCredentials))
   //}

   let auth_cookie = match generate_auth_cookie(&email) {
    Ok(cookie) => cookie,
    Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
   };

   let updated_jar = jar.add(auth_cookie);

   (updated_jar, Ok(StatusCode::OK))
    
}

#[derive(Deserialize)]
pub struct LoginRequest{
    email: String,
    password: String

}

