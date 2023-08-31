use std::path::PathBuf;

use tauri::api::path::document_dir;
use wpilog::log::{CreateDataLogConfig, DataLog, DataLogDaemon, OpenDataLogConfig};

use crate::{error::EnokiError, enoki_types::EnokiValue};

use super::DATALOG;

static RELATIVE_DIRECTORY: &str = "Enoki/Datalogs";

pub fn setup_directory() -> Result<(), ()> {
    if let Some(docu_path) = document_dir() {
        let datalog_dir = docu_path.join(RELATIVE_DIRECTORY);
        if !datalog_dir.exists() {
            let res = std::fs::create_dir_all(datalog_dir);
            if res.is_err() {
                tracing::error!("Failed to create datalog directory: {}", res.unwrap_err());
                return Err(());
            }
        }
        Ok(())
    } else {
        Err(())
    }
}

pub fn create_datalog_daemon() -> DataLogDaemon {
    if let Err(_) = setup_directory() {
        tracing::error!("Failed to setup datalog directory");
        panic!();
    }

    let current_time_string =
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string() + ".wpilog";

    let relative_path = format!("{}/{}", RELATIVE_DIRECTORY, current_time_string);

    let abs_path = document_dir()
        .unwrap()
        .join(relative_path)
        .to_str()
        .unwrap()
        .to_string();

    let config = CreateDataLogConfig {
        file_path: abs_path.into(),
        metadata: "".into(),
    };

    //if can't create datalog crash
    let datalog = DataLog::create(config).expect("Failed to create datalog");
    datalog.as_daemon()
}

pub fn start_datalog_entry(
    name: &str,
    entry_type: &str,
    metadata: Option<&str>,
) -> Result<(), EnokiError> {
    DATALOG.lock().borrow_sender().start_entry(
        String::from(name),
        String::from(entry_type),
        metadata.map(String::from),
    )?;
    Ok(())
}

pub fn end_datalog_entry(name: &str) -> Result<(), EnokiError> {
    DATALOG
        .lock()
        .borrow_sender()
        .finish_entry(String::from(name))?;
    Ok(())
}

pub fn log_datalog_value(name: &str, value: EnokiValue) -> Result<(), EnokiError> {
    DATALOG
        .lock()
        .borrow_sender()
        .append_to_entry(String::from(name), value.into())?;
    Ok(())
}

pub fn open_datalog(path: PathBuf) -> Result<DataLog, EnokiError> {
    let config = OpenDataLogConfig {
        file_path: path,
        io_type: wpilog::log::IOType::ReadOnly,
    };
    let datalog = DataLog::open(config)?;
    Ok(datalog)
}
