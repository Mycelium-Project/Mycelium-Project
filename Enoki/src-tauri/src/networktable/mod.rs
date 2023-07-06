
#[macro_use]
pub mod handler;
pub mod ffi;
// pub mod python_funcs;

use std::collections::HashMap;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use ffi::tauri::*;

use self::handler::{NetworkTableClientId, NetworkTableClient};

pub static NETWORK_CLIENT_MAP: Lazy<Mutex<HashMap<NetworkTableClientId, NetworkTableClient>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));


pub fn networktable_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("nt")
        .setup(|_app_handle| {tracing::info!("Setting up networktable plugin"); Ok(())})
        .invoke_handler(tauri::generate_handler![
            start_network_table_client,
            is_network_table_client_stopped,
            stop_network_table_client,
            subscribe_to_topic,
            unsubscribe_from_topic,
            get_subbed_data,
            get_subbed_data_with_history,
            set_topic_value,
            unpublish_topic
        ])
        .build()
}