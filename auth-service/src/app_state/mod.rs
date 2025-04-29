use std::sync::Arc;
use tokio::sync::RwLock;

use crate::
    domain::{
        user_stores::UserStore,
        email_client::EmailClient,
        data_stores::{BannedTokenStore, TwoFACodeStore }
    };

pub type UserStoreType = Arc<RwLock<dyn UserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore>>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore>>;
pub type EmailClientType = Arc<RwLock<dyn EmailClient>>;

#[derive(Clone)]
pub struct AppState{
    pub user_store: UserStoreType,
    pub banned_tokens_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub email_client: EmailClientType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_tokens_store: BannedTokenStoreType,
        two_fa_code_store: TwoFACodeStoreType,
        email_client: EmailClientType 
    ) -> Self {
        Self {
            user_store,
            banned_tokens_store,
            two_fa_code_store, 
            email_client,
        }
    }
}



