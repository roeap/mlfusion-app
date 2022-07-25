use serde::{Deserialize, Serialize};

mod models {
    include!("flight_fusion.ipc.v1alpha1.rs");
    include!("flight_fusion.ipc.v1alpha1.serde.rs");
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Parameters {
    pub name: String,
    pub info: String,
}

#[tauri::command]
pub fn greet(source: models::AreaSourceReference) -> String {
    format!("Hello, {:?}!", source.table)
}
