use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::user::{self, User}};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> impl IntoResponse {
    let user = User::new(
        request.email.clone(),
        request.password.clone(),
        request.requires_2fa
    );

    let mut user_store = state.user_store.write().await;
    let _ = user_store.users.insert(request.email, user);

    let response = Json(
        SignupResponse{message: "User created successfully".to_string()}
    );



    (StatusCode::CREATED, response)
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