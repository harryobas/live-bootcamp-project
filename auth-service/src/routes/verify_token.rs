use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};

use serde::Deserialize;

use crate::{app_state::AppState, domain::error::AuthAPIError, utils::auth::validate_token};



pub async fn verify_token(
    State(_state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;

    if validate_token(&token).await.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    Ok(StatusCode::OK)

}


#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}