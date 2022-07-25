use crate::models;

#[tauri::command]
pub fn list_data_assets(source: models::AreaSourceReference) -> String {
    format!("Hello, {:?}!", source.table)
}
