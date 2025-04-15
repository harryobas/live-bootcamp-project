use super::helpers::TestApp;

#[tokio::test]
async fn verify_token_returns_success() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!(
        {
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
        }
    );
    let response = app.verify_token(request_body).await;

    assert_eq!(response.status().as_u16(), 200);
}