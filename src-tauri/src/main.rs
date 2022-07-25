#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod api;
mod clients;
mod errors;
mod models {
    include!("generated/flight_fusion.ipc.v1alpha1.rs");
    include!("generated/flight_fusion.ipc.v1alpha1.serde.rs");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api::list_data_assets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
