// Copyright 2025 Project Edoba
// SPDX-License-Identifier: Apache-2.0
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

// Entry point for Tauri-based Gymnast application.
// Sets up the window and runs the Tauri event loop.
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("Gymnast PDF Viewer").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application - get your hat on Sherlock, it's time to go debugging.");
}
