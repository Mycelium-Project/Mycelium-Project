pub mod commands;
pub mod handler;

use handler::CLIWrapper;
use parking_lot::Mutex;

use commands::*;

pub static CLI_WRAPPERS: Mutex<Vec<CLIWrapper>> = Mutex::new(Vec::new());

pub fn terminal_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("terminal")
        .setup(|_app_handle| {
            tracing::info!("Setting up terminal plugin");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_terminal,
            kill_terminal,
            write_terminal,
            read_out_terminal,
            read_err_terminal,
            is_running_terminal
        ])
        .build()
}
