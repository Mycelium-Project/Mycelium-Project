// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::Ipv4Addr;

mod network_table_handler;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_network_table_handler])
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
  if network_table_handler::nt4(Ipv4Addr::from(address), port).is_err() {
      println!("Error in network_table_handler::nt4()");
  }
}

// TODO: Add other functions listed in NT4Handler.tsx for export and in network_table_handler.rs