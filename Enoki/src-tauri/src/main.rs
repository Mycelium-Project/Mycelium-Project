// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::{Ipv4Addr, SocketAddrV4};
use std::cell::RefCell;
use std::collections::HashMap;
use tokio::task::JoinHandle as TokioJoinHandle;

mod network_table_handler;

thread_local! {

    static THREAD_POOL: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    static NETWORK_CLIENT_MAP: RefCell<HashMap<SocketAddrV4, TokioJoinHandle<()>>> = RefCell::new(HashMap::new());
}


#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_network_table_handler, stop_network_table_handler])
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
fn start_network_table_handler(address: [u8; 4], port: u16) {
    let ip = Ipv4Addr::from(address);

    if let Some(thread) = NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().remove(&SocketAddrV4::new(ip, port))) {
        tracing::info!("Stopping network table handler for {}:{}", ip, port);
        thread.abort();
    }

    let thread = THREAD_POOL.with(|pool| {
        pool.spawn(async move {
            match network_table_handler::nt4(ip, port).await {
                Ok(_) => println!("Network table handler started successfully"),
                Err(e) => println!("Error starting network table handler: {}", e),
            }
        })});
    NETWORK_CLIENT_MAP.with(|map| {
        map.borrow_mut().insert(SocketAddrV4::new(ip, port), thread);
    });
}


#[tauri::command]
fn stop_network_table_handler(address: [u8; 4], port: u16) {
    let ip = Ipv4Addr::from(address);
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(thread) = map.borrow_mut().remove(&SocketAddrV4::new(ip, port)) {
            tracing::info!("Stopping network table handler for {}:{}", ip, port);
            thread.abort();
        }
    });
}

// TODO: Add other functions listed in NT4Handler.ts for export and in network_table_handler.rs
