use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::{user::User, user_stores::{UserStore, UserStoreError}, Email, Password};

#[derive(Debug,Clone, Default)]
pub struct HashMapUserStore {
    pub users: Arc<RwLock<HashMap<Email, User>>>
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError> {
        let mut users = self.users.write().await;
        if users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);

        }
        users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.read().await.get(&Email(email.to_string())) {
            None => Err(UserStoreError::UserNotFound),
            Some(user) => Ok(user.clone()),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let email = Email(email.to_string());
        let password = Password(password.to_string());
        match self.users.read().await.get(&email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                }else{
                    Err(UserStoreError::InvalidCredentials)
                }
            },
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let email = Email("user@example.com".to_string());
        let password = Password("password123".to_string());
        let requires_2fa = true;

        let usr = User::new(email, password, requires_2fa);
        let map = Arc::new(RwLock::new(HashMap::new()));

        let  store = HashMapUserStore {
            users: map
        };

        assert!(store.add_user(usr).await.is_ok());

    }

    #[tokio::test]
    async fn test_get_user() {
        let email = Email("user@example.com".to_string());
        let password = Password("password123".to_string());
        let requires_2fa = true;

        let usr = User::new(
            email.clone(),
            password,
            requires_2fa,
        );

        let  map = Arc::new(RwLock::new(HashMap::new()));
        let _ = map.write().await.insert(email.clone(), usr);

        let store = HashMapUserStore {users: map};
        assert!(store.get_user(email.as_ref()).await.is_ok());
    }
    #[tokio::test]
    async fn test_validate_user() {
        let email = Email("user@example.com".to_string());
        let password = Password("password123".to_string());
        let requires_2fa = true;

        let usr = User::new(
            email.clone(),
            password.clone(),
            requires_2fa,
        );

        let  map = Arc::new(RwLock::new(HashMap::new()));
        let _ = map.write().await.insert(email.clone(), usr);
        let store = HashMapUserStore {
            users: map
        };

        assert!(store.validate_user(email.as_ref(), password.as_ref()).await.is_ok())

    }
}