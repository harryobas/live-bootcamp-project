
use super::helpers::{TestApp, get_random_email};
use auth_service::routes::signup::SignupResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "$password",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email,
            "password": "password123"
        }),
        serde_json::json!({
            "email": &random_email,
            "requires2FA": true
        })

    ];

    for case in test_cases.iter() {
        let response = app.post_signup(case.clone()).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}", case
        )
    }

}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let request_body = serde_json::json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(request_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully".to_string()
    };

    assert_eq!(
        response.json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body"),
        expected_response
    )

}