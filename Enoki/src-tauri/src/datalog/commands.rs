use wpilog::log::DatalogEntryResponse;

use crate::{error::{log_result, EnokiError}, DATALOG};

use super::handler::open_datalog;


#[tauri::command]
pub fn read_datalog(path: String) -> Result<Vec<DatalogEntryResponse>, EnokiError> {
    let datalog = log_result(open_datalog(path.into()))?;
    let entries = datalog.get_all_entries();
    Ok(entries)
}

#[tauri::command]
pub fn retrieve_dl_daemon_data() -> Vec<DatalogEntryResponse> {
    DATALOG.with(|datalog| datalog.borrow_mut().get_all_entries().clone())
}