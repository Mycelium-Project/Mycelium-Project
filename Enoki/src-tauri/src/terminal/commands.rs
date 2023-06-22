use std::path::PathBuf;

use super::{handler::CLIWrapper, CLI_WRAPPERS};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TerminalId {
    id: u32
}

#[tauri::command]
pub async fn start_terminal(exe_path: String, args: Vec<String>) -> Result<TerminalId, String> {
    let mut cli_wrapper = CLIWrapper::new(PathBuf::from(exe_path), args);
    let res = cli_wrapper.run();
    if res.is_err() {
        tracing::error!("Failed to start cli: {:?}", res.unwrap_err());
        return Err("Failed to start cli".to_string());
    }
    let id = {
        let mut cli_wrappers = CLI_WRAPPERS.lock();
        cli_wrappers.push(cli_wrapper);
        cli_wrappers.len() as u32
    };
    Ok(TerminalId { id })
}

#[tauri::command]
pub async fn kill_terminal(id: TerminalId) {
    let mut cli_wrappers = CLI_WRAPPERS.lock();
    if let Some(cli_wrapper) = cli_wrappers.get_mut(id.id as usize - 1) {
        let res = cli_wrapper.kill();
        if res.is_err() {
            tracing::error!("Failed to kill cli: {:?}", res.unwrap_err());
        }
    }
}

#[tauri::command]
pub async fn write_terminal(id: TerminalId, command: String) -> Result<(), String> {
    let mut cli_wrappers = CLI_WRAPPERS.lock();
    if let Some(cli_wrapper) = cli_wrappers.get_mut(id.id as usize - 1) {
        let res = cli_wrapper.write_command(&command);
        if res.is_err() {
            tracing::error!("Failed to write to cli: {:?}", res.unwrap_err());
            return Err("Failed to write to cli".to_string());
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn read_out_terminal(id: TerminalId) -> Result<String, String> {
    let mut cli_wrappers = CLI_WRAPPERS.lock();
    if let Some(cli_wrapper) = cli_wrappers.get_mut(id.id as usize - 1) {
        let res = cli_wrapper.read_output();
        if res.is_err() {
            tracing::error!("Failed to read from cli: {:?}", res.unwrap_err());
            return Err("Failed to read from cli".to_string());
        }
        return Ok(res.unwrap());
    }
    Err("Failed to read from cli".to_string())
}

#[tauri::command]
pub async fn read_err_terminal(id: TerminalId) -> Result<String, String> {
    let mut cli_wrappers = CLI_WRAPPERS.lock();
    if let Some(cli_wrapper) = cli_wrappers.get_mut(id.id as usize - 1) {
        let res = cli_wrapper.read_error();
        if res.is_err() {
            tracing::error!("Failed to read from cli: {:?}", res.unwrap_err());
            return Err("Failed to read from cli".to_string());
        }
        return Ok(res.unwrap());
    }
    Err("Failed to read from cli".to_string())
}

#[tauri::command]
pub async fn is_running_terminal(id: TerminalId) -> bool {
    let mut cli_wrappers = CLI_WRAPPERS.lock();
    if let Some(cli_wrapper) = cli_wrappers.get_mut(id.id as usize - 1) {
        return cli_wrapper.is_running();
    }
    false
}