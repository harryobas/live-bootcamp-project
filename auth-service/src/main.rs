use auth_service::Application;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000";
    let app = Application::build(addr)
        .await
        . expect("Failed to build app");


    app.run().await.expect("Failed to run app");

}
