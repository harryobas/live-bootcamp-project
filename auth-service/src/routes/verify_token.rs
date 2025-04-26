use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};

use serde::Deserialize;

use crate::{app_state::AppState, domain::error::AuthAPIError, utils::auth::validate_token};



pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;
    let banned_store = state.banned_tokens_store;

     validate_token(&token, banned_store.clone()).await.map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(StatusCode::OK)

}


#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}