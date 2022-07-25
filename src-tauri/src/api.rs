use serde::{Deserialize, Serialize};

use crate::models::{self, area_source_reference::Table};

#[derive(Serialize, Deserialize)]
pub struct AreaInfo {
    pub source: models::AreaSourceReference,
}

#[tauri::command]
pub fn list_data_assets() -> Vec<AreaInfo> {
    let area_ref = models::AreaSourceReference {
        table: Some(Table::Location(models::AreaTableLocation {
            areas: vec!["Hello".to_string()],
            name: "World".to_string(),
        })),
    };
    vec![AreaInfo { source: area_ref }]
}
