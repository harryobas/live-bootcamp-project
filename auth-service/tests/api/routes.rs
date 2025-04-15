use super::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html")
}

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
#[tokio::test]
async fn logout_returns_success() {
    let app = TestApp::new().await;
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}
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

