use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::{
    Email, 
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError}
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: Arc<RwLock<HashMap<Email, (LoginAttemptId, TwoFACode)>>>,
}
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes
            .write()
            .await
            .insert(email, (login_attempt_id, code));

        Ok(())
    }

    async fn remove_code(&self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes
            .write()
            .await
            .remove(email)
            .ok_or(TwoFACodeStoreError::UnexpectedError)
            .map(|_| ())
    }

    async fn get_code(
        &self,
        email: &Email
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes
            .read()
            .await
            .get(email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
            .map(|v|(v.0.clone(), v.1.clone()))
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let email = Email("user@example.com".to_string());
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let codes = Arc::new(RwLock::new(HashMap::new()));

        let  hashmap_two_fa_code_store = HashmapTwoFACodeStore{codes};

        assert!(
            hashmap_two_fa_code_store
                .add_code(email, login_attempt_id, code)
                .await
                .is_ok()
        );

    }

    #[tokio::test]
    async fn test_remove_code() {
        let email = Email("user@example.com".to_string());
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let codes = Arc::new(RwLock::new(HashMap::new()));

        let  hashmap_two_fa_code_store = HashmapTwoFACodeStore{codes};
        let _ = hashmap_two_fa_code_store.add_code(email.clone(), login_attempt_id, code).await;

        assert!(
            hashmap_two_fa_code_store.remove_code(&email).await.is_ok()
        );
    }

    #[tokio::test]
    async fn test_get_code() {
        let email = Email("user@example.com".to_string());
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let codes = Arc::new(RwLock::new(HashMap::new()));

        let  hashmap_two_fa_code_store = HashmapTwoFACodeStore{codes};
        let _ = hashmap_two_fa_code_store.add_code(email.clone(), login_attempt_id, code).await;

        assert!(
            hashmap_two_fa_code_store.get_code(&email).await.is_ok()
        );

    }

}