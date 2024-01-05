// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nix::sys;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// gets uname from system
#[tauri::command]
fn uname() -> String {
    let utsname = sys::utsname::uname();
    format!(
        "sysname: {:?}, nodename: {:?}, release: {:?}, version: {:?}, machine: {:?}",
        utsname.unwrap().machine(),
        utsname.unwrap().nodename(),
        utsname.unwrap().release(),
        utsname.unwrap().sysname(),
        utsname.unwrap().version(),
    )
}

// opens vscode
#[tauri::command]
fn open_vscode() {
    let _ = std::process::Command::new("code")
        .arg(".")
        .spawn()
        .expect("failed to open vscode");
}

// opens default browser in macos
#[tauri::command]
fn open_browser() {
    let _ = std::process::Command::new("open")
        .arg("https://x.com")
        .spawn()
        .expect("failed to open browser");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![uname])
        .invoke_handler(tauri::generate_handler![open_vscode])
        .invoke_handler(tauri::generate_handler![open_browser])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
