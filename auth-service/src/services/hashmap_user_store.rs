use std::collections::HashMap;

use crate::domain::{user::User, user_stores::{UserStore, UserStoreError}, Email, Password};

#[derive(Debug,Clone, Default)]
pub struct HashMapUserStore {
    pub users: HashMap<Email, User>
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);

        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(&Email(email.to_string())) {
            None => Err(UserStoreError::UserNotFound),
            Some(user) => Ok(user.clone()),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let email = Email(email.to_string());
        let password = Password(password.to_string());
        match self.users.get(&email) {
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
        let users = HashMap::new();

        let  mut store = HashMapUserStore {users};

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

        let  mut users = HashMap::new();
        let _ = users.insert(email.clone(), usr);

        let store = HashMapUserStore {users};
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

        let  mut users = HashMap::new();
        let _ = users.insert(email.clone(), usr);
        let store = HashMapUserStore {users};

        assert!(store.validate_user(email.as_ref(), password.as_ref()).await.is_ok())

    }
}