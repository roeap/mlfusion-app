use serde::{Deserialize, Serialize};

use crate::clients::mlfusion::MLFusionClient;
use crate::models;

#[derive(Serialize, Deserialize)]
pub struct AreaInfo {
    pub source: models::AreaSourceReference,
}

#[tauri::command]
pub async fn list_data_assets() -> Result<Vec<AreaInfo>, String> {
    Ok(MLFusionClient::try_new("localhost", 50051)
        .await
        .map_err(|err| err.to_string())?
        .clone()
        .list_data_assets()
        .await
        .map_err(|err| err.to_string())?
        .into_iter()
        .map(|info| AreaInfo { source: info })
        .collect())
}
