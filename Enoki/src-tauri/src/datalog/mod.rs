
use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use wpilog::log::DataLogDaemon;

pub mod commands;
pub mod handler;

pub static DATALOG: Lazy<Arc<Mutex<DataLogDaemon>>> = Lazy::new(|| Arc::new(Mutex::new(handler::create_datalog_daemon())));