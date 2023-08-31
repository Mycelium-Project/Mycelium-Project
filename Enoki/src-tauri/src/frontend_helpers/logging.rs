
#[tauri::command]
pub async fn tracing_frontend(level: String, msg: String, line: String, file: String) {
    match level.as_str() {
        "trace" => tracing::trace!(source = "frontend", message = msg, line = line, file = file),
        "debug" => tracing::debug!(source = "frontend", message = msg, line = line, file = file),
        "info" => tracing::info!(source = "frontend", message = msg, line = line, file = file),
        "warn" => tracing::warn!(source = "frontend", message = msg, line = line, file = file),
        "error" => tracing::error!(source = "frontend", message = msg, line = line, file = file),
        _ => tracing::error!("Invalid log level: {}", level),
    }
}