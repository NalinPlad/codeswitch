// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// Imports
use std::process::Command;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]

async fn get_mac() -> String {
    let defa

    let ifc = String::from_utf8(
        Command::new("ifconfig")
            .arg("en0")
            .output()
            .expect("ifconfig failed")
            .stdout
    ).unwrap();

    let 

    return ifc;

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
