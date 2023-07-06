
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use wpilog::log::DataLogDaemon;

pub mod tauri_cmds;
pub mod handler;

use tauri_cmds::*;

pub static DATALOG: Lazy<Mutex<DataLogDaemon>> = Lazy::new(|| Mutex::new(handler::create_datalog_daemon()));

pub fn datalog_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("datalog")
        .setup(|_app_handle| {tracing::info!("Setting up datalog plugin"); Ok(())})
        .invoke_handler(tauri::generate_handler![
            read_datalog,
            retrieve_dl_daemon_data,
            send_mark
        ])
        .build()
}