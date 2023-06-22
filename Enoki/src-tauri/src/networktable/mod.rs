
#[macro_use]
pub mod tauri_cmds;
pub mod handler;
// pub mod python_funcs;

use std::{collections::HashMap, sync::Arc};

use tauri_cmds::*;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use self::handler::{NetworkTableClientId, NetworkTableClient};

pub static NETWORK_CLIENT_MAP: Lazy<Arc<Mutex<HashMap<NetworkTableClientId, NetworkTableClient>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));


pub fn networktable_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("nt")
        .setup(|_app_handle| {tracing::info!("Setting up networktable plugin"); Ok(())})
        .invoke_handler(tauri::generate_handler![
            start_network_table_client,
            does_network_table_client_exist,
            stop_network_table_client,
            subscribe_to_topic,
            // set_boolean_topic,
            // set_float_topic,
            // set_double_topic,
            // set_string_topic,
            // set_int_topic,
            // set_boolean_array_topic,
            // set_float_array_topic,
            // set_double_array_topic,
            // set_string_array_topic,
            // set_int_array_topic,
            // get_subbed_entries_values,
            // get_subbed_entry_value,
            // get_client_timestamp
        ])
        .build()
}