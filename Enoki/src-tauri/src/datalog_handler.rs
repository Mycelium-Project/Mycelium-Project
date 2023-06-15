use std::path::PathBuf;

use tauri::{Runtime, plugin::TauriPlugin, RunEvent, api::path::document_dir};
use wpilog_rs::log::{DataLogDaemon, DataLog, CreateDataLogConfig};

use crate::{DATALOG, mushroom_types::MushroomTypes, NETWORK_CLIENT_MAP};

static RELATIVE_DIRECTORY: &str = "Enoki/Datalogs";

pub fn setup_directory() -> Result<(), ()> {
    if let Some(docu_path)  = document_dir() {
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


pub(crate) fn create_datalog_daemon() -> DataLogDaemon {
    if let Err(_) = setup_directory() {
        tracing::error!("Failed to setup datalog directory");
        panic!();
    }

    let currunt_time_string = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
        + ".wpilog";

    let relative_path = format!("{}/{}", RELATIVE_DIRECTORY, currunt_time_string);

    let abs_path = document_dir().unwrap().join(relative_path).to_str().unwrap().to_string();

    tracing::info!("Creating datalog at {}", PathBuf::from(abs_path.clone()).display());

    let config = CreateDataLogConfig {
        file_path: abs_path.into(),
        metadata: "".into()
    };

    //if can't create datalog crash
    let datalog = DataLog::create(config.clone()).unwrap();
    datalog.as_daemon()
}

pub(crate) fn make_dl_plugin<R: Runtime>() -> TauriPlugin<R> {
    let sender = DATALOG.with(|datalog| {
        let snd = datalog.borrow().get_sender();
        snd.start_entry(String::from("/ConnectedClients"), "string[]".into(), None).ok();
        snd
    });
    tauri::plugin::Builder::new("datalog")
        .on_event(move |_app_handle, event| {
            match event {
                RunEvent::MainEventsCleared => {
                    tracing::info!("MainEventsCleared");
                    let connected_clients = NETWORK_CLIENT_MAP.with(|map| {
                        map.borrow().keys().map(|k| k.repr()).collect::<Vec<String>>()
                    });
                    sender.append_to_entry(String::from("/ConnectedClients"),
                        MushroomTypes::StringArray(connected_clients).into()).ok();
                },
                _ => {}
            }
        })
        .build()
}