use super::helpers::{TestApp, get_random_email};
use auth_service::{
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
    routes::login::TwoFactorAuthResponse,
    domain::Email
};


#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let request_body = serde_json::json!({
        "password": "password",
        //email: "user@example.com"
    });

    let response = app.post_login(request_body).await;
    assert_eq!(response.status().as_u16(), 422);
}
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let request_body = serde_json::json!({
        "email": &random_email,
        "password": "pass"
    });

    let response = app.post_login(request_body).await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let response_body = serde_json::json!({
        "email": &random_email,
        "password": "$password1234"
    });

    let response = app.post_login(response_body).await;
    assert_eq!(response.status().as_u16(), 401);

}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let _response = app.post_signup(signup_body).await;
    //assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(login_body).await;
    assert_eq!(response.status().as_u16(), 200);
  
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());


}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let password = "password123";

    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true
    });

    app.post_signup(signup_body).await;

    let login_body = serde_json::json!({
        "email": email,
        "password": password
    });

    let login_response = app.post_login(login_body).await;
    assert_eq!(login_response.status().as_u16(), 206);
  

    let response_body = login_response
            .json::<TwoFactorAuthResponse>()
            .await
            .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_body.message,  "2FA required".to_owned());

    let (stored_code, _) = app
        .app_state
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email(email.clone()))
        .await
        .expect("2FA code not found in store");

    assert_eq!(stored_code.as_ref(), response_body.login_attempt_id);


}



    


