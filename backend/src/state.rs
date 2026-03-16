use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
 
#[derive(Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub active: bool,
}
 
impl Device {
    pub fn new(ip: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            ip,
            active: true,
        }
    }
}
 
#[derive(Clone, Default)]
pub struct AppState {
    pub devices: Arc<RwLock<HashMap<Uuid, Device>>>,
}
 