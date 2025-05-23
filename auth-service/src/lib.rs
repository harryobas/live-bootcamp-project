pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

use std::error::Error;

use routes::{
    signup::signup, 
    login::login,
    logout::logout, 
    verify_token::verify_token,
    verify_2fa::verify_2fa
};
use app_state::AppState;
use domain::error::AuthAPIError;
use serde::{Serialize, Deserialize};
use axum::{
    http::{StatusCode, Method}, 
    response::IntoResponse, 
    routing::post, serve::Serve, Router,
    Json,
};

use axum::response::Response;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub struct Application {
    pub server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?

        ];
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);


        Ok(Self { server, address })

    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }


}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
            AuthAPIError::InCorrectCredentials => (StatusCode::UNAUTHORIZED, "Not authorized"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "missing token"),

        };
        let body = Json(ErrorResponse{
            error: error_message.to_string(),
        });
        (status, body).into_response()

    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {

    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
}




