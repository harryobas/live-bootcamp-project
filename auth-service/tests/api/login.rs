use super::helpers::TestApp;


#[tokio::test]
async fn login_returns_success() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!(
        {
            "email": "user@example.com",
            "password": "$password"
        }
    );

    let response = app.login(request_body).await;
    assert_eq!(response.status().as_u16(), 200);
}