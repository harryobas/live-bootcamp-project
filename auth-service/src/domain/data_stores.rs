#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&self, token: &str);
    async fn is_banned_token(&self, token: &str) -> bool;
}
