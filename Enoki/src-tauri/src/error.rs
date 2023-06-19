use serde::Serialize;
use tauri::api::path::document_dir;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum EnokiError {
    #[error("DataLog io error: {0:?}")]
    DlIo(String),
    #[error("DataLog error: {0:?}")]
    Dl(#[from] wpilog::DatalogError),
    #[error("NT error: {0:?}")]
    NTTimeout(#[from] network_tables::NetworkTablesError),
    #[error("Not main thread: {0:?}")]
    NotMainThread(String),
    #[error("No valid address found")]
    NoValidAddress,
}

#[inline(always)]
/// Logs the error if there is one
pub fn log_result<T, E: std::error::Error>(result: Result<T, E>) -> Result<T, E> {
    match &result {
        Err(err) => {
            tracing::error!("{}", err)
        }
        _ => {}
    };
    result
}

#[inline(always)]
/// Consumes the result and logs the error if there is one
pub fn log_result_consume<T, E: std::error::Error>(result: Result<T, E>) {
    match &result {
        Err(err) => {
            tracing::error!("{}", err)
        }
        _ => {}
    }
}

pub struct TraceWriter {
    buffer: Vec<u8>,
    file: std::fs::File,
}

impl TraceWriter {
    pub fn new() -> Self {
        let currunt_time_string =
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string() + ".debuglog.txt";

        let file_path = document_dir()
            .unwrap()
            .join("Enoki/DebugLogs")
            .join(currunt_time_string);

        if !file_path.exists() {
            std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        };

        let file = std::fs::File::create(file_path).unwrap();

        Self {
            buffer: Vec::new(),
            file,
        }
    }
}

impl std::io::Write for TraceWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
