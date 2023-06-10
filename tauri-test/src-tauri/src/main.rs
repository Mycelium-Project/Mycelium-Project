// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod network_table_handler;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_network_table_handler])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

#[tauri::command]
fn start_network_table_handler() {
  if network_table_handler::nt4().is_err() {
      println!("Error in network_table_handler::nt4()");
  }
}

// TODO: Add other functions listed in NT4Handler.tsx