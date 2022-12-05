#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("plot", |app, req| {
            use tauri::http::ResponseBuilder;
            ResponseBuilder::new()
                .header("Content-Type", "application/octet-stream")
                .header("Access-Control-Allow-Origin", "null")
                .header("Access-Control-Allow-Methods", "*")
                .body(vec![1; 128])
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
