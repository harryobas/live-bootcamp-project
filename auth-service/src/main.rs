
use std::sync::Arc;

use auth_service::{
    app_state::AppState, 
    services::{hashmap_user_store::HashMapUserStore, hashset_banned_token_store::HashSetBannedTokenStore},
    Application,
    utils::constants::prod
 };

#[tokio::main]
async fn main() {
    let user_store = HashMapUserStore::default();
    let banned_tokens_store = HashSetBannedTokenStore::default();

    let app_state = AppState {
        user_store: Arc::new(user_store.clone()),
        banned_tokens_store: Arc::new(banned_tokens_store.clone())

    };

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        . expect("Failed to build app");


    app.run().await.expect("Failed to run app");

}
