use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Parameters {
    pub name: String,
    pub info: String,
}

#[tauri::command]
pub fn greet(params: Parameters) -> String {
    format!("Hello, {} - {}!", params.name, params.info)
}
