// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::{Emitter, EventTarget};

#[tauri::command]
fn trigger_vue_function(app: tauri::AppHandle) {
    app.emit_to(EventTarget::any(), "vue-event", "from rust");
}

#[tauri::command]
fn newWindow1CallRust(name: String) -> String {
    println!("{}",name);
    format!("Hello, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app| {

        Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![newWindow1CallRust,trigger_vue_function])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}