// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::{EnokiError, TraceWriter};
use mushroom_types::{MushroomEntry, MushroomPath, MushroomValue};
use network_tables::v4::SubscriptionOptions;
use networktable::handler::{
    get_connect_client_names, NetworkTableClient, NetworkTableClientId, SubscriptionPackage,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::thread;
use tauri::plugin::TauriPlugin;
use tauri::{RunEvent, Runtime};
use tracing::metadata::LevelFilter;
use wpilog::log::{DataLogDaemon, DatalogEntryResponse};

use crate::datalog::handler::{create_datalog_daemon, log_datalog_value, start_datalog_entry};
use crate::error::log_result_consume;
use crate::mushroom_types::MushroomTable;
use crate::networktable::handler::start_nt4_client;

use crate::datalog::commands::{read_datalog};

mod error;
pub mod mushroom_types;

#[cfg(test)]
mod test;

#[macro_use]
pub mod datalog;
pub mod networktable;

thread_local! {

    static THREAD_POOL: RefCell<Option<tokio::runtime::Runtime>> = RefCell::new(
        Some(tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()));

    static NETWORK_CLIENT_MAP: RefCell<HashMap<NetworkTableClientId, NetworkTableClient>> = RefCell::new(HashMap::new());

    static DATALOG: RefCell<DataLogDaemon> = RefCell::new(create_datalog_daemon());
}

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
            .with_writer(non_blocking_file)
            .init();
        _guard_lock = _guard_file;
    }

    tauri::Builder::default()
        .plugin(backend_plugin())
        .invoke_handler(tauri::generate_handler![
            start_network_table_client,
            stop_network_table_client,
            does_network_table_client_exist,
            subscribe_to_topic,
            set_boolean_topic,
            set_float_topic,
            set_double_topic,
            set_string_topic,
            set_int_topic,
            set_boolean_array_topic,
            set_float_array_topic,
            set_double_array_topic,
            set_string_array_topic,
            set_int_array_topic,
            get_subbed_entries_values,
            get_client_timestamp,
            get_subbed_entry_value,
            retrieve_dl_daemon_data,
            read_datalog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn backend_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("backend_plugin")
        .on_event(move |_app_handle, event| match event {
            RunEvent::MainEventsCleared => {
                per_frame();
            }
            RunEvent::ExitRequested { .. } => {
                close();
            }
            RunEvent::Ready => {
                init();
            }
            _ => {}
        })
        .build()
}

///anything put in this will run once per frame of the ui, keep it light
/// WARNING: only called while window is focused
/// if you need something to run in the background *at all times* use a thread
fn per_frame() {
    log_result_consume(log_datalog_value(
        "/ClientsConnected",
        MushroomValue::StringArray(get_connect_client_names()),
    ));
}

///called when the ui first starts up
fn init() {
    tracing::info!("Init");
    log_result_consume(start_datalog_entry(
        "/ClientsConnected",
        "string[]",
        Some("Clients running from the app"),
    ));
}

///called when the app is shutting down
fn close() {
    tracing::info!("Closing");
    DATALOG.with(|daemon| daemon.borrow_mut().kill());
    THREAD_POOL.with(|pool| (pool.replace(None)).unwrap().shutdown_background());
    NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().clear());
}

fn check_if_main_thread() -> Result<(), EnokiError> {
    if thread::current().name().unwrap_or_default() != "main" {
        return Err(EnokiError::NotMainThread(String::from(
            thread::current().name().unwrap_or_default(),
        )));
    }
    Ok(())
}

/**
* Starts the network table handler
*
* address The IP address of the network table server as an array of 4 bytes
* in typescript pass in an array of 4 numbers
*
* port The port of the network table server as a 16-bit unsigned integer
* in typescript pass in a number
*/
#[tauri::command]
fn start_network_table_client(
    address: [u8; 4],
    port: u16,
    identity: String,
) -> NetworkTableClientId {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableClientId::new(ip, port, identity.clone());

    if let Some(client) = NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().remove(&id)) {
        tracing::info!("Stopping network table client for {}", id);
        client.stop();
    }

    tracing::info!("Starting network table client for {}", id);
    let client = start_nt4_client(ip, port, identity).unwrap();

    NETWORK_CLIENT_MAP.with(|map| {
        map.borrow_mut().insert(id.clone(), client);
    });

    return id;
}

#[tauri::command]
fn does_network_table_client_exist(client_id: NetworkTableClientId) -> bool {
    NETWORK_CLIENT_MAP.with(|map| map.borrow().contains_key(&client_id))
}

#[tauri::command]
fn stop_network_table_client(client_id: NetworkTableClientId) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().remove(&client_id) {
            tracing::info!("Stopping network table cleint for {}", client_id);
            client.stop();
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn subscribe_to_topic(
    client_id: NetworkTableClientId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let data = SubscriptionPackage::new(
                topic.clone(),
                SubscriptionOptions {
                    all,
                    prefix,
                    periodic,
                    ..Default::default()
                },
            );
            client.subscribe(vec![data]);
            tracing::info!("Subscribed to topic {}", topic);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_boolean_topic(client_id: NetworkTableClientId, topic: String, value: bool) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry =
                MushroomEntry::new(MushroomValue::Boolean(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_float_topic(client_id: NetworkTableClientId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(MushroomValue::Float(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set float topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_double_topic(client_id: NetworkTableClientId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry =
                MushroomEntry::new(MushroomValue::Double(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set double topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_string_topic(client_id: NetworkTableClientId, topic: String, value: String) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::String(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set string topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_int_topic(client_id: NetworkTableClientId, topic: String, value: i64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(MushroomValue::Int(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set int topic {} to {} for {}", topic, value, client_id);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_boolean_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<bool>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::BooleanArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_float_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::FloatArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set float array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_double_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::DoubleArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set double array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_string_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<String>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::StringArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set string array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn set_int_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<i64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::IntArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set int array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
fn get_subbed_entries_values(client_id: NetworkTableClientId) -> MushroomTable {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            tracing::info!("Getting subbed entries values for {}", client_id);
            client.poll()
        } else {
            tracing::warn!("No network table client found for {}", client_id);
            MushroomTable::new(0)
        }
    })
}

#[tauri::command]
fn get_subbed_entry_value(
    client_id: NetworkTableClientId,
    path: MushroomPath,
) -> Option<MushroomEntry> {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            tracing::info!("Getting subbed entry value for {}", client_id);
            client.poll().get_entry(&path)
        } else {
            tracing::warn!("No network table client found for {}", client_id);
            None
        }
    })
}

#[tauri::command]
fn get_client_timestamp(client_id: NetworkTableClientId) -> f64 {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            tracing::info!("Getting client timestamp for {}", client_id);
            client.poll().get_timestamp() as f64 / 1000000_f64
        } else {
            tracing::warn!("No network table client found for {}", client_id);
            0_f64
        }
    })
}

#[tauri::command]
fn retrieve_dl_daemon_data() -> Vec<DatalogEntryResponse> {
    DATALOG.with(|datalog| datalog.borrow_mut().get_all_entries().clone())
}
