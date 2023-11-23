// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Imports
use std::{os::unix::process::CommandExt, process::Command};
use tauri::Manager;

#[tauri::command]
async fn get_mac() -> String {
    let default_interface = String::from_utf8(
        Command::new("route")
            .arg("get")
            .arg("default")
            .output()
            .expect("route failed")
            .stdout,
    )
    .unwrap();

    let interface_split: Vec<&str> = default_interface.split("interface: ").collect();
    let default_interface = interface_split[1].split("\n").collect::<Vec<&str>>()[0];

    println!("{:?}", default_interface);

    let ifc = String::from_utf8(
        Command::new("ifconfig")
            .arg(default_interface)
            .output()
            .expect("ifconfig failed")
            .stdout,
    )
    .unwrap();

    let mac_addr = ifc.split("ether ").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .split("\n")
        .collect::<Vec<&str>>()[0];
    return mac_addr.to_string();
    //     .split(" ")
    //     .collect::<Vec<&str>>()[0];

    // return mac_addr.to_string();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            // app.get_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
            // app.get_window("main").unwrap().maximize().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_mac])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
