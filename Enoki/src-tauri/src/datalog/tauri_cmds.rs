use wpilog::log::{DatalogEntryResponse, DataLogValue};

use crate::{error::{log_result, EnokiError, log_result_consume}, enoki_types::{TimestampedEnokiValue, EnokiObject, now, EnokiValue, EnokiKey}};

use super::{handler::open_datalog, DATALOG};


#[tauri::command]
pub fn read_datalog(path: String) -> Result<Vec<DatalogEntryResponse>, EnokiError> {
    let datalog = log_result(open_datalog(path.into()))?;
    let entries = datalog.get_all_entries();
    Ok(entries)
}

#[tauri::command]
pub fn retrieve_dl_daemon_data() -> EnokiObject {
    let mut obj = EnokiObject::new(now());
    let mut dl = DATALOG.lock();
    let entries = dl.get_all_entries();
    for entry in entries {
        let key = EnokiKey::from(entry.name.clone());
        let mut history = Vec::new();
        entry.marks.iter().for_each(|mark| {
            history.push(TimestampedEnokiValue::new(mark.timestamp, EnokiValue::from(mark.value.clone())));
        });
        obj.set_history(&key, history)
    }
    obj
}

#[tauri::command]
pub fn send_mark(field: String, value: TimestampedEnokiValue) {
    let mut datalog = DATALOG.lock();
    let already_exist = datalog.summary().contains_key(&field);
    if already_exist {
        log_result_consume(
            datalog.borrow_sender()
                .append_to_entry_with_timestamp(field, value.value.into(), value.timestamp));
    } else {
        let dl_val = DataLogValue::from(value.value);
        log_result_consume(
            datalog.borrow_sender().start_entry(
                field.clone(),
                dl_val.get_data_type(),
                Some("{ source: \"frontend\"}".to_string()))
            );
        log_result_consume(
            datalog.borrow_sender()
                .append_to_entry_with_timestamp(field, dl_val, value.timestamp));
    }
}