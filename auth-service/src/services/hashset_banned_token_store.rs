use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::data_stores::BannedTokenStore;


#[derive(Debug, Clone, Default) ]
pub struct HashSetBannedTokenStore {
    banned_tokens: Arc<RwLock<HashSet<String>>>
}
#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add_token(&self, token: &str) {
        let token = String::from(token);
        let mut banned_tokens = self.banned_tokens.write().await;
        banned_tokens.insert(token);
    }

    async fn is_banned_token(&self, token: &str) -> bool {
        self.banned_tokens.read().await.contains(token)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let banned_token = "token";

        let banned_tokens = Arc::new(RwLock::new(HashSet::new()));
        let hashset_store = HashSetBannedTokenStore {
            banned_tokens
        };

        hashset_store.add_token(banned_token).await;
        assert!(hashset_store.banned_tokens.read().await.len() > 0);
    }

    #[tokio::test]
    async fn test_is_banned_token() {
        let banned_token = "token";

        let banned_tokens = Arc::new(RwLock::new(HashSet::new()));
        let hashset_store = HashSetBannedTokenStore {
            banned_tokens
        };

        hashset_store.add_token(banned_token).await;
        assert_eq!(hashset_store.is_banned_token(banned_token).await, true);

    }
}

