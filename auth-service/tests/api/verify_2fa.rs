use super::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_returns_success() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!(
        {
            "email": "user@example.com",
            "loginAttemptId": "abc123",
            "2FACode": "654321"
        }
          
    );
    let response = app.verify_2fa(request_body).await;
    assert_eq!(response.status().as_u16(), 200);
}