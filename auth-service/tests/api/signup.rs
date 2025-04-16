use super::helpers::{TestApp, get_random_email};

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