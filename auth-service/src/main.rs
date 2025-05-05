
use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState, get_postgres_pool, services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore,
         hashmap_user_store::HashMapUserStore, 
         hashset_banned_token_store::HashSetBannedTokenStore,
         mock_email_client::MockEmailClient
    }, utils::constants::{prod, DATABASE_URL}, Application
 };

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let user_store = Arc::new(
        RwLock::new(HashMapUserStore::default())
    );
    let banned_tokens_store = Arc::new(
        RwLock::new(HashSetBannedTokenStore::default())
    );
    let two_fa_code_store = Arc::new(
        RwLock::new(HashmapTwoFACodeStore::default())
    );
    let email_client = Arc::new(
        RwLock::new(MockEmailClient{})
    );


    let app_state = AppState::new(
        user_store,
        banned_tokens_store,
        two_fa_code_store,
        email_client,

    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        . expect("Failed to build app");

    app.run().await.expect("Failed to run app");

}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create postgres connection pool");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
