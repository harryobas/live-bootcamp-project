use super::helpers::TestApp;


#[tokio::test]
async fn signup_returns_success() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!(
        {
            "email": "user@example.com",
            "password": "$password",
            "requires2FA": true
        }
    );

    let response = app.signup(request_body).await;
    assert_eq!(response.status().as_u16(), 201);

    
}