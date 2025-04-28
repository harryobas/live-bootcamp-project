
use std::sync::Arc;

use auth_service::{
    app_state::AppState, 
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore,
         hashmap_user_store::HashMapUserStore, 
         hashset_banned_token_store::HashSetBannedTokenStore
    }, 
    utils::constants::prod, Application
 };

#[tokio::main]
async fn main() {
    let user_store = Arc::new(HashMapUserStore::default());
    let banned_tokens_store = Arc::new(HashSetBannedTokenStore::default());
    let two_fa_code_store = Arc::new(HashmapTwoFACodeStore::default());

    let app_state = AppState::new(user_store, banned_tokens_store, two_fa_code_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        . expect("Failed to build app");


    app.run().await.expect("Failed to run app");

}
