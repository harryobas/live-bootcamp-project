
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::{data_stores::{LoginAttemptId, TwoFACode}, error::AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>
) ->  Result<(CookieJar, Response), AuthAPIError> {
    let email =  Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let password =  Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.clone();

   user_store
    .read()
    .await
    .validate_user(email.as_ref(), password.as_ref())
    .await
    .map_err(|_| AuthAPIError::InCorrectCredentials)?;
    
   let user = user_store
    .read()
    .await
    .get_user(email.as_ref())
    .await
    .map_err(|_| AuthAPIError::InCorrectCredentials)?;

    match user.requires_2fa {
        true => return handle_2fa(&email, &state, jar).await,
        false => return handle_no_2fa(&user.email, jar).await,
    }
  
    
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar
) -> Result<(CookieJar, Response), AuthAPIError> {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();
    
    state.two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let recipient = email;
    let subject = "2FA Code";
    let content= two_fa_code.as_ref();

    state.email_client.write().await.send_mail(
        recipient, 
        subject, 
        content
    ).await.map_err(|_| AuthAPIError::UnexpectedError)?;

    let response_body = Json(LoginResponse::TwoFactorAuth(
        TwoFactorAuthResponse { 
            message:"2FA required".to_string(),
            login_attempt_id: login_attempt_id.as_ref().to_string(),
        }
    ));
       

    Ok((jar, (StatusCode::PARTIAL_CONTENT, response_body).into_response()))

}

async fn handle_no_2fa(email: &Email, jar: CookieJar) -> Result<(CookieJar, Response), AuthAPIError> {
    generate_auth_cookie(email)
        .map(|cookie| {
            let updated_jar = jar.add(cookie);
            (updated_jar, Json(LoginResponse::RegularAuth).into_response())

        })
        .map_err(|_| AuthAPIError::UnexpectedError)
    }

    

#[derive(Deserialize)]
pub struct LoginRequest{
    email: String,
    password: String

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

