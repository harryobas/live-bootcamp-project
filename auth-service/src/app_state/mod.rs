use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{domain::user_stores::UserStore, services::hashmap_user_store::HashMapUserStore};

pub type UserStoreType = Arc<RwLock<HashMapUserStore>>;

#[derive(Clone)]
pub struct AppState{
    pub user_store: Arc<dyn UserStore>,
}



