// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use datalog::handler::log_datalog_value;
use error::TraceWriter;
use enoki_types::EnokiValue;
use networktable::handler::get_connect_client_names;

use tauri::plugin::TauriPlugin;
use tauri::{RunEvent, Runtime};
use tracing_subscriber::prelude::*;

// use crate::datalog::handler::start_datalog_entry;
// use crate::datalog::DATALOG;
// use crate::datalog::handler::{create_datalog_daemon, log_datalog_value, start_datalog_entry};
use crate::error::log_result_consume;
use crate::frontend_helpers::logging::tracing_frontend;
use crate::networktable::NETWORK_CLIENT_MAP;

mod error;
pub mod enoki_types;

#[cfg(test)]
mod test;

// pub mod datalog;
pub mod frontend_helpers;
pub mod networktable;
pub mod python_helpers;
// pub mod terminal;
pub mod robot_interface;
pub mod logging;

fn main() {
    // guard lock needs to live till end of program
    let (non_blocking_std_io, _guard_std_io) =
    tracing_appender::non_blocking(std::io::stdout());
    let (non_blocking_file, _guard_file) = tracing_appender::non_blocking(TraceWriter::new());
    let _guard_lock = (_guard_std_io, _guard_file);

    //use multiple tracing layers so traces can be sent to multiple places
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_file)
                .event_format(logging::CompactFormatter))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_std_io)
                .event_format(logging::PrettyFormatter)
        )
        .init();

    tauri::Builder::default()
        .plugin(backend_plugin())
        .plugin(networktable::networktable_plugin())
        // .plugin(datalog::datalog_plugin())
        // .plugin(terminal::terminal_plugin())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_store::Builder::default().build())
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
                exit();
            }
            RunEvent::WindowEvent { label, event, .. } => {
                match event.to_owned() {
                    tauri::WindowEvent::CloseRequested { .. } => {
                        tracing::info!("Window {} closed", label);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![tracing_frontend])
        .build()
}

///called when the ui first starts up
pub fn init() {
    tracing::info!("Init");
    // log_result_consume(
    //     start_datalog_entry(
    //         "/ClientsConnected",
    //         "string[]",
    //         Some("Clients running from the app"),
    //     ),
    // );
}

/// Anything put in this will run once per frame of the ui, keep it light
/// 
/// WARNING: only called while window is focused
/// if you need something to run in the background *at all times* use a thread
fn per_frame() {
    // log_result_consume(
    //     log_datalog_value(
    //         "/ClientsConnected",
    //         EnokiValue::StringArray(get_connect_client_names()),
    //     ),
    // );
}

///called when the app is shutting down
fn exit() {
    tracing::info!("Closing");
    // DATALOG.lock().kill();
    NETWORK_CLIENT_MAP.lock().clear();
}
