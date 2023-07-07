// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use datalog::handler::log_datalog_value;
use error::TraceWriter;
use enoki_types::EnokiValue;
use networktable::handler::get_connect_client_names;

use tauri::plugin::TauriPlugin;
use tauri::{RunEvent, Runtime};
use tracing::metadata::LevelFilter;

use crate::datalog::handler::start_datalog_entry;
use crate::datalog::DATALOG;
// use crate::datalog::handler::{create_datalog_daemon, log_datalog_value, start_datalog_entry};
use crate::error::log_result_consume;
use crate::frontend_helpers::logging::tracing_frontend;
use crate::networktable::NETWORK_CLIENT_MAP;

mod error;
pub mod enoki_types;

#[cfg(test)]
mod test;

pub mod datalog;
pub mod frontend_helpers;
pub mod networktable;
pub mod python_helpers;
pub mod terminal;

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
            .with_ansi(false)
            .with_writer(non_blocking_file)
            .init();
        _guard_lock = _guard_file;
    }

    tauri::Builder::default()
        .plugin(backend_plugin())
        .plugin(frontend_helpers::appvars_plugin())
        .plugin(networktable::networktable_plugin())
        .plugin(datalog::datalog_plugin())
        .plugin(terminal::terminal_plugin())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn backend_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("native")
        .on_event(move |_app_handle, event| match event {
            RunEvent::Ready => {
                init()
            }
            RunEvent::MainEventsCleared => {
                per_frame();
            }
            RunEvent::ExitRequested { .. } => {
                close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![tracing_frontend])
        .build()
}

///called when the ui first starts up
pub fn init() {
    tracing::info!("Init");
    log_result_consume(
        start_datalog_entry(
            "/ClientsConnected",
            "string[]",
            Some("Clients running from the app"),
        ),
    );
}

///anything put in this will run once per frame of the ui, keep it light
/// WARNING: only called while window is focused
/// if you need something to run in the background *at all times* use a thread
fn per_frame() {
    log_result_consume(
        log_datalog_value(
            "/ClientsConnected",
            EnokiValue::StringArray(get_connect_client_names()),
        ),
    );
}

///called when the app is shutting down
fn close() {
    tracing::info!("Closing");
    DATALOG.lock().kill();
    NETWORK_CLIENT_MAP.lock().clear();
}
