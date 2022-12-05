#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::{read, canonicalize};

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("plot", |app, req| {
            use tauri::http::ResponseBuilder;

            let data = read(canonicalize("../public/fake-nebula.bin").unwrap()).unwrap();

            ResponseBuilder::new()
                .header("Content-Type", "application/octet-stream")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Method", "*")
                .body(data)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
