

use auth_service::{
    app_state::AppState,
    services::{hashmap_user_store::HashMapUserStore, hashset_banned_token_store::HashSetBannedTokenStore},
    Application,
    utils::constants::test
};
use reqwest::cookie:: Jar;
use uuid::Uuid;
use std::sync::Arc;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub app_state: AppState,
    
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashMapUserStore::default();
        let banned_tokens_store = HashSetBannedTokenStore::default();
        let app_state = AppState{
            user_store: Arc::new(user_store.clone()),
            banned_tokens_store: Arc::new(banned_tokens_store.clone())
        };

        let app = Application::build(app_state.clone(),test::APP_ADDRESS)
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
             cookie_jar,
            http_client,
            app_state,
        }


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

    pub async fn post_login(&self, body: impl serde::Serialize) ->  reqwest::Response {
         self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Faild to execute POST /login request")
    }

    pub async fn post_verify_2fa(&self, body: impl serde::Serialize) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Faild to execute POST /verify-2fa request")

    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute POST /logout reqiest")
    }

    pub async fn post_verify_token(&self, body: impl serde::Serialize) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute POST /verify-token request")
    }


}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}