#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{command, Window};

#[command]
fn hide_window(window: Window) {
    window.hide().unwrap();
}

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hide_window])
        .run(ctx)
        .expect("error running application");
}
