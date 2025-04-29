
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState, 
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore,
         hashmap_user_store::HashMapUserStore, 
         hashset_banned_token_store::HashSetBannedTokenStore,
         mock_email_client::MockEmailClient
    }, 
    utils::constants::prod, Application
 };

#[tokio::main]
async fn main() {
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
