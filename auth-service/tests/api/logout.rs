use super::helpers::{get_random_email, TestApp};

use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing(){
    let app = TestApp::new().await;

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
    
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME

        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL")
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
    
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";

    app.post_signup(serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false
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
    .expect("No token found")
    .value()
    .to_string();

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let banned_tokens = app.app_state.banned_tokens_store;
    assert_eq!(
        banned_tokens
        .read()
        .await
        .is_banned_token(&token)
        .await, 
        true
    );



}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false
    });

    let signup_response = app.post_signup(signup_body).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let login_response = app.post_login(login_body).await;
    assert_eq!(login_response.status().as_u16(), 200);
    assert!(login_response.headers().contains_key("set-cookie"));

    let logout_res_1 = app.post_logout().await;
    assert_eq!(logout_res_1.status().as_u16(), 200);

    let logout_res_2 = app.post_logout().await;
    assert_eq!(logout_res_2.status().as_u16(), 400);

}