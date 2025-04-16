

use auth_service::Application;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self {address, http_client,}


    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute GET / request")
    }

    pub async fn post_signup(&self, body: impl serde::Serialize) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute POST /signup request")
    }

    pub async fn login(&self, body: impl serde::Serialize) ->  reqwest::Response {
        let body_str = serde_json::to_string(&body).expect("Failed to serialize body");

        self.http_client
            .post(&format!("{}/login", &self.address))
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .await
            .expect("Faild to execute POST /login request")
    }

    pub async fn verify_2fa(&self, body: impl serde::Serialize) -> reqwest::Response {
        let body_str = serde_json::to_string(&body).expect("Failed to serialize body");


        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .await
            .expect("Faild to execute POST /verify-2fa request")

    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .header("Cookie", "jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")
            .send()
            .await
            .expect("Failed to execute POST /logout reqiest")
    }

    pub async fn verify_token(&self, body: impl serde::Serialize) -> reqwest::Response {
        let body_str = serde_json::to_string(&body).expect("Failed to serialize body");

        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .body(body_str)
            .send()
            .await
            .expect("Failed to execute POST /verify-token request")
    }


}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}