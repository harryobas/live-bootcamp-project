
use super::helpers::{TestApp, get_random_email};
use auth_service::utils::constants::JWT_COOKIE_NAME;


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!({});

    let response = app.post_verify_token(request_body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false
    });

    let _ = app.post_signup(signup_body).await;

    let login_body = serde_json::json!({
        "email": random_email,
        "password": password
    });

    let login_response = app.post_login(login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);

    let token  = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No cookie found")
        .value()
        .to_string();

    let response = app.post_verify_token(serde_json::json!({
        "token": token.clone()
    }))
    .await;

    assert_eq!(response.status().as_u16(), 200);
  
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!({
        "token": "invalid token"
    });

    let response = app.post_verify_token(request_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";

    app.post_signup(serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false,
    }))
    .await;

    let login_response = app.post_login(serde_json::json!({
        "email": random_email,
        "password": password,
    }))
    .await;

    let token = login_response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("No cookie found")
        .value()
        .to_string();

    app.post_logout().await;

    let response = app.post_verify_token(serde_json::json!({
        "token": token
    })).await;

    assert_eq!(response.status().as_u16(), 401);
}