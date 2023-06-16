// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::{EnokiError, TraceWriter};
use mushroom_types::MushroomValue;
use networktable::handler::{
    get_connect_client_names, NetworkTableClient, NetworkTableClientId,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::thread;
use tauri::plugin::TauriPlugin;
use tauri::{RunEvent, Runtime};
use tracing::metadata::LevelFilter;
use wpilog::log::DataLogDaemon;

use crate::datalog::handler::{create_datalog_daemon, log_datalog_value, start_datalog_entry};
use crate::error::log_result_consume;

use crate::datalog::commands::*;
use crate::networktable::commands::*;


mod error;
pub mod mushroom_types;

#[cfg(test)]
mod test;

#[macro_use]
pub mod datalog;
pub mod networktable;

thread_local! {

    static THREAD_POOL: RefCell<Option<tokio::runtime::Runtime>> = RefCell::new(
        Some(tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()));

    static NETWORK_CLIENT_MAP: RefCell<HashMap<NetworkTableClientId, NetworkTableClient>> = RefCell::new(HashMap::new());

    static DATALOG: RefCell<DataLogDaemon> = RefCell::new(create_datalog_daemon());
}

#[tokio::main]
async fn main() {
    // guard lock needs to live till end of program
    let _guard_lock;
    if cfg!(debug_assertions) {
        let (non_blocking_std_io, _guard_std_io) =
            tracing_appender::non_blocking(std::io::stdout());
        tracing_subscriber::fmt()
            .with_file(true)
            .with_thread_names(true)
            .pretty()
            .with_line_number(true)
            .without_time()
            .with_level(true)
            .with_writer(non_blocking_std_io)
            .init();
        _guard_lock = _guard_std_io;
    } else {
        let (non_blocking_file, _guard_file) = tracing_appender::non_blocking(TraceWriter::new());
        tracing_subscriber::fmt()
            .with_file(true)
            .with_thread_names(true)
            .with_line_number(true)
            .with_level(true)
            .with_max_level(LevelFilter::WARN)
            .with_writer(non_blocking_file)
            .init();
        _guard_lock = _guard_file;
    }

    tauri::Builder::default()
        .plugin(backend_plugin())
        .invoke_handler(tauri::generate_handler![
            start_network_table_client,
            stop_network_table_client,
            does_network_table_client_exist,
            subscribe_to_topic,
            set_boolean_topic,
            set_float_topic,
            set_double_topic,
            set_string_topic,
            set_int_topic,
            set_boolean_array_topic,
            set_float_array_topic,
            set_double_array_topic,
            set_string_array_topic,
            set_int_array_topic,
            get_subbed_entries_values,
            get_client_timestamp,
            get_subbed_entry_value,
            retrieve_dl_daemon_data,
            read_datalog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn backend_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("backend_plugin")
        .on_event(move |_app_handle, event| match event {
            RunEvent::MainEventsCleared => {
                per_frame();
            }
            RunEvent::ExitRequested { .. } => {
                close();
            }
            RunEvent::Ready => {
                init();
            }
            _ => {}
        })
        .build()
}

///anything put in this will run once per frame of the ui, keep it light
/// WARNING: only called while window is focused
/// if you need something to run in the background *at all times* use a thread
fn per_frame() {
    log_result_consume(log_datalog_value(
        "/ClientsConnected",
        MushroomValue::StringArray(get_connect_client_names()),
    ));
}

///called when the ui first starts up
fn init() {
    tracing::info!("Init");
    log_result_consume(start_datalog_entry(
        "/ClientsConnected",
        "string[]",
        Some("Clients running from the app"),
    ));
}

///called when the app is shutting down
fn close() {
    tracing::info!("Closing");
    DATALOG.with(|daemon| daemon.borrow_mut().kill());
    THREAD_POOL.with(|pool| (pool.replace(None)).unwrap().shutdown_background());
    NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().clear());
}

/// used in non-command functions that use thread loacal variables
/// commands are guranteed to be called in the scope of the main thread
fn check_if_main_thread() -> Result<(), EnokiError> {
    if thread::current().name().unwrap_or_default() != "main" {
        return Err(EnokiError::NotMainThread(String::from(
            thread::current().name().unwrap_or_default(),
        )));
    }
    Ok(())
}


