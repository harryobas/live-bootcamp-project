use std::sync::Arc;

use crate::
    domain::{
        user_stores::UserStore,
        data_stores::{BannedTokenStore, TwoFACodeStore }
    };

pub type UserStoreType = Arc<dyn UserStore>;
pub type BannedTokenStoreType = Arc<dyn BannedTokenStore>;
pub type TwoFACodeStoreType = Arc<dyn TwoFACodeStore>;

#[derive(Clone)]
pub struct AppState{
    pub user_store: UserStoreType,
    pub banned_tokens_store: Arc<dyn BannedTokenStore>,
    pub two_fa_code_store: Arc<dyn TwoFACodeStore>
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_tokens_store: BannedTokenStoreType,
        two_fa_code_store: TwoFACodeStoreType, 
    ) -> Self {
        Self {
            user_store,
            banned_tokens_store,
            two_fa_code_store, 
        }
    }
}



