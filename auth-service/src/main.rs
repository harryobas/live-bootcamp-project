
use std::sync::Arc;

use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashMapUserStore, Application
 };

#[tokio::main]
async fn main() {
    let user_store = HashMapUserStore::default();
    let app_state = AppState {user_store: Arc::new(user_store.clone())};

    let addr = "0.0.0.0:3000";
    let app = Application::build(app_state, addr)
        .await
        . expect("Failed to build app");


    app.run().await.expect("Failed to run app");

}
