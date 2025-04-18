use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::services::hashmap_user_store::HashMapUserStore;

pub type UserStoreType = Arc<RwLock<HashMapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}

pub fn get_user_store() -> UserStoreType {
    let users = HashMap::new();
    let store = HashMapUserStore{users};

    Arc::new(RwLock::new(store))
}