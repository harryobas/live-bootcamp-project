use crate::helpers::get_random_email;
use super::helpers::TestApp;

use auth_service::{
    domain::{data_stores::{LoginAttemptId, TwoFACode}, Email}, 
    ErrorResponse
};


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let request_body = serde_json::json!(
        {
            "email": random_email,
            "loginAttemptId": "abc123",
        }
          
    );
    let response = app.post_verify_2fa(request_body).await;
    assert_eq!(response.status().as_u16(), 422)
}

#[tokio::test]
async fn should_return_400_if_invalid_input(){
    let app = TestApp::new().await;

    let email = get_random_email();
    let login_attempt_id = "a-valid-uuid-v4";
    let two_fa_code = "12a";

    let response_body = serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code
    });

    let response = app.post_verify_2fa(response_body).await;
    assert_eq!(response.status().as_u16(), 400);

    let error_body = response.json::<ErrorResponse>().await.unwrap();
    assert_eq!(error_body.error, "Invalid credentials");

}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let email = get_random_email();

    let login_attempt_id = LoginAttemptId::default().0;
    let two_fa_code = TwoFACode::default().0;

    let request_body = serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code,
    });

    let response = app.post_verify_2fa(request_body).await;
    assert_eq!(response.status().as_u16(), 401);

    let error_body = response.json::<ErrorResponse>().await.unwrap();
    assert_eq!(error_body.error, "Not authorized");
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = "$password";

    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true
    });

    let _signup_response = app.post_signup(signup_body).await;

    let _login_response = app.post_login(serde_json::json!({
        "email": email,
        "password": password
    })) 
    .await;

    let code_store = app.app_state.two_fa_code_store.clone();
    let (login_attempt_id, two_fa_code) = code_store
        .read()
        .await
        .get_code(&Email(email.clone()))
        .await
        .unwrap();
    let response = app.post_verify_2fa(serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": two_fa_code.as_ref()
    }))
    .await;

    assert_eq!(response.status().as_u16(), 200);

   let cookie_header = response
    .headers()
    .get("set-cookie")
    .expect("Expected 'set-cookie' header to be present");


    let cookie_str = cookie_header.to_str().unwrap();
    assert!(cookie_str.contains("jwt"));
      
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {    
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = "password123";

    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true
    });

    app.post_signup(signup_body).await;

    app.post_login(serde_json::json!({
        "email": email,
        "password": password
    }))
    .await;

    let  code_store = app.app_state.two_fa_code_store.clone();
    let (login_attempt_id, two_fa_code) = code_store
        .read()
        .await
        .get_code(&Email(email.clone()))
        .await
        .unwrap();

    let response_1 = app.post_verify_2fa(serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": two_fa_code.as_ref()

    }))
    .await;

    assert_eq!(response_1.status().as_u16(), 200);

    let response_2 = app.post_verify_2fa(serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode": two_fa_code.as_ref()

    }))
    .await;

    assert_eq!(response_2.status().as_u16(), 401);

    let error_body = response_2.json::<ErrorResponse>().await.unwrap();
    assert_eq!(error_body.error, "Not authorized");


}

