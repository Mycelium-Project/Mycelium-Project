// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mushroom_types::{MushroomEntry, MushroomTable, MushroomTypes};
use network_table_handler::{NetworkTableHandler, NetworkTableHandlerId, SubscriptionPackage};
use network_tables::v4::SubscriptionOptions;
use std::time::Instant;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::Ipv4Addr;

pub mod mushroom_types;
mod network_table_handler;

thread_local! {

    static THREAD_POOL: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    static NETWORK_CLIENT_MAP: RefCell<HashMap<NetworkTableHandlerId, NetworkTableHandler>> = RefCell::new(HashMap::new());
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_network_table_handler,
            stop_network_table_handler,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn timestamp() -> f64 {
    let now = Instant::now();
    let secs = now.elapsed().as_secs_f64();
    secs
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
fn start_network_table_handler(
    address: [u8; 4],
    port: u16,
    identity: String,
) -> NetworkTableHandlerId {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableHandlerId::new(ip, port, identity.clone());

    if let Some(handler) = NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().remove(&id)) {
        tracing::info!("Stopping network table handler for {}:{}", ip, port);
        handler.stop();
    }

    tracing::info!("Starting network table handler for {}:{}", ip, port);
    let handler = network_table_handler::nt4(ip, port, identity).unwrap();

    NETWORK_CLIENT_MAP.with(|map| {
        map.borrow_mut().insert(id.clone(), handler);
    });

    return id;
}

#[tauri::command]
fn stop_network_table_handler(address: [u8; 4], port: u16) {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableHandlerId::new(ip, port, "".to_string());
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().remove(&id) {
            tracing::info!("Stopping network table handler for {}:{}", ip, port);
            handler.stop();
        }
    });
}

#[tauri::command]
fn subscribe_to_topic(
    handler_id: NetworkTableHandlerId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let data = SubscriptionPackage::new(
                topic.clone(),
                SubscriptionOptions {
                    all,
                    prefix,
                    periodic,
                    ..Default::default()
                },
            );
            handler.subscribe(vec![data]);
            tracing::info!("Subscribed to topic {}", topic);
        }
    });
}

#[tauri::command]
fn set_boolean_topic(handler_id: NetworkTableHandlerId, topic: String, value: bool) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Boolean(value), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set boolean topic {} to {}", topic, value);
        }
    });
}

#[tauri::command]
fn set_float_topic(handler_id: NetworkTableHandlerId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Float(value), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set float topic {} to {}", topic, value);
        }
    });
}

#[tauri::command]
fn set_double_topic(handler_id: NetworkTableHandlerId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Double(value), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set double topic {} to {}", topic, value);
        }
    });
}

#[tauri::command]
fn set_string_topic(handler_id: NetworkTableHandlerId, topic: String, value: String) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::String(value), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set string topic {} to {}", topic, value);
        }
    });
}

#[tauri::command]
fn set_int_topic(handler_id: NetworkTableHandlerId, topic: String, value: i64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::Int(value), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set int topic {} to {}", topic, value);
        }
    });
}

#[tauri::command]
fn set_boolean_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<bool>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::BooleanArray(value.clone()), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set boolean array topic {} to {:?}", topic, value);
        }
    });
}

#[tauri::command]
fn set_float_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::FloatArray(value.clone()), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set float array topic {} to {:?}", topic, value);
        }
    });
}

#[tauri::command]
fn set_double_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::DoubleArray(value.clone()), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set double array topic {} to {:?}", topic, value);
        }
    });
}

#[tauri::command]
fn set_string_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<String>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry = MushroomEntry::new(
                MushroomTypes::StringArray(value.clone()), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set string array topic {} to {:?}", topic, value);
        }
    });
}

#[tauri::command]
fn set_int_array_topic(handler_id: NetworkTableHandlerId, topic: String, value: Vec<i64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(handler) = map.borrow_mut().get_mut(&handler_id) {
            let entry =
                MushroomEntry::new(MushroomTypes::IntArray(value.clone()), MushroomEntry::make_path(topic.as_str()), Some(timestamp()));
            handler.publish(vec![entry]);
            tracing::info!("Set int array topic {} to {:?}", topic, value);
        }
    });
}
