use std::collections::HashMap;

use crate::domain::user::{self, User};

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,

}

pub struct HashMapUserStore {
    pub users: HashMap<String, User>
}

impl HashMapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(&user.email) {
            Some(_user) => return Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
            },

        }

        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            None => Err(UserStoreError::UserNotFound),
            Some(user) => Ok(user.clone()),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
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
        let email = "user@example.com".to_string();
        let password = "password123".to_string();
        let requires_2fa = true;

        let usr = User::new(email, password, requires_2fa);
        let map: HashMap<String, User> = HashMap::new();

        let mut store = HashMapUserStore {
            users: map
        };

        assert!(store.add_user(usr).is_ok());

    }

    #[tokio::test]
    async fn test_get_user() {
        let email = "user@example.com".to_string();
        let password = "password123".to_string();
        let requires_2fa = true;

        let usr = User::new(
            email.clone(),
            password,
            requires_2fa,
        );

        let mut map = HashMap::new();
        let _ = map.insert(email.clone(), usr);

        let store = HashMapUserStore {users: map};
        assert!(store.get_user(&email).is_ok());
    }
    #[tokio::test]
    async fn test_validate_user() {
        let email = "user@example.com".to_string();
        let password = "password123".to_string();
        let requires_2fa = true;

        let usr = User::new(
            email.clone(),
            password.clone(),
            requires_2fa,
        );

        let mut map = HashMap::new();
        let _ = map.insert(email.clone(), usr);
        let store = HashMapUserStore {
            users: map
        };

        assert!(store.validate_user(&email, &password).is_ok())

    }
}