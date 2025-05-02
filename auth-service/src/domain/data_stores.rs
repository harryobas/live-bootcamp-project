use super::Email;
use rand::Rng;

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: &str);
    async fn is_banned_token(&self, token: &str) -> bool;
}

#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode
    ) -> Result<(), TwoFACodeStoreError>;

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginAttemptId(pub String);

impl LoginAttemptId {
    pub fn parse(id: &str) -> Result<Self, String> {
        uuid::Uuid::parse_str(id)
            .map(|uuid| Self(uuid.to_string()))
            .map_err(|_| "Inavalid id format".to_string()) 
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        LoginAttemptId(id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TwoFACode(pub String);

impl TwoFACode {
   pub fn parse(code: &str) -> Result<Self, String> {
        if code.len() == 6 {
            Ok(Self(code.to_string()))
        }else {
            Err("Invalid code format".to_string())
        }

    }  
}

impl Default for TwoFACode {
    fn default() -> Self {
        let  code: u32 = rand::rng().random_range(100_000..1_000_000);
        Self(code.to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

