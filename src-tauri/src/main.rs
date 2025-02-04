// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod handler;

use core::setup;
use handler::hello;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![hello::greet])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running coding-hui/wetill application");
}
