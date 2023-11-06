// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pnet::datalink::interfaces;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dashboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_dashboard() -> String {
    interfaces()
        .iter()
        .into_iter()
        .map(|i| i.name.clone())
        .collect::<Vec<String>>()
        .join(",")
}
