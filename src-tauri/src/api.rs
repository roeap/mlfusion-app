mod models {
    include!("flight_fusion.ipc.v1alpha1.rs");
    include!("flight_fusion.ipc.v1alpha1.serde.rs");
}

#[tauri::command]
pub fn greet(source: models::AreaSourceReference) -> String {
    format!("Hello, {:?}!", source.table)
}
