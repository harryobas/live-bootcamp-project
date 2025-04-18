
use auth_service::{
    Application, 
    app_state::{AppState, get_user_store},
 };

#[tokio::main]
async fn main() {
    let user_store = get_user_store();
    let app_state = AppState::new(user_store);

    let addr = "0.0.0.0:3000";
    let app = Application::build(app_state, addr)
        .await
        . expect("Failed to build app");


    app.run().await.expect("Failed to run app");

}
