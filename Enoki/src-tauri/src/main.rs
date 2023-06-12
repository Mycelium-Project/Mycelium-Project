// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mushroom_types::MushroomTable;
use network_table_handler::{NetworkTableHandlerId, NetworkTableHandler};
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};

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
            stop_network_table_handler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
fn start_network_table_handler(address: [u8; 4], port: u16, identity: String) -> NetworkTableHandlerId {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableHandlerId::new(ip, port, identity.clone());

    if let Some(handler) =
        NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().remove(&id))
    {
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

// TODO: Add other functions listed in NT4Handler.ts for export and in network_table_handler.rs
