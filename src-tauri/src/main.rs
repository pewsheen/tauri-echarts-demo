#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use crossbeam_utils::atomic::AtomicCell;
use gui_handle::GuiHandle;

fn main() {
    let mut handle = GuiHandle::new().unwrap();
    let data_hdl = handle.start(1000, 0.1).unwrap();

    // Share latest chunk between tokio task and URL scheme handler.
    let latest_chunk = Arc::new(AtomicCell::new(None));
    let chunk = latest_chunk.clone();

    tauri::Builder::default()
        .setup(|_app| {
            // Start a tokio task when start up.
            tauri::async_runtime::spawn(async move {
                let mut data_hdl = data_hdl;
                let mut idx = 0;

                while let Some(chunk) = data_hdl.wait_chunk_n(idx).await {
                    assert_eq!(idx, chunk.idx(), "Chunk Index Mismatch?");
                    // Update lastest chunk
                    latest_chunk.store(Some(chunk));

                    idx += 1;
                }
            });
            Ok(())
        })
        // Register an URL scheme called plot. We don't care what the request is.
        // Just need to send the necessary dataset in the chunk.
        .register_uri_scheme_protocol("plot", move |_app, _req| {
            use tauri::http::ResponseBuilder;

            let data = if let Some(chunk) = chunk.take() {
                let fft = chunk.sample_fft();
                // FIXME Should we use to_le_bytes instead?
                let slice =
                    unsafe { std::slice::from_raw_parts(fft.as_ptr() as *const u8, fft.len() * 4) };
                slice.to_vec()
            } else {
                vec![]
            };

            ResponseBuilder::new()
                .header("Content-Type", "application/octet-stream")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Method", "*")
                .body(data)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
