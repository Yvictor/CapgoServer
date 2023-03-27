use poem::Result;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct AppStats {
    pub version_name: String,
    pub action: String,
    pub version_build: String,
    pub version_code: String,
    pub version_os: String,
    pub plugin_version: String,
    pub platform: String,
    pub app_id: String,
    pub device_id: String,
    pub custom_id: Option<String>,
    pub is_prod: Option<bool>,
    pub is_emulator: Option<bool>,
}

#[derive(Debug, Deserialize, Object)]
pub struct Stats {
    pub status: String,
}

// #[derive(Debug, Serialize, Object)]
// pub struct ErrorResponse {
//     pub error: String,
//     pub message: String,
// }

#[derive(ApiResponse)]
pub enum StatsResponse {
    #[oai(status = 200)]
    Stats(Json<Stats>),
    // #[oai(status = 404)]
    // Error(Json<ErrorResponse>),
}

pub async fn handle_stats_request(app_stats: &AppStats) -> Result<StatsResponse> {
    info!("app_stats: {:?}", app_stats);
    // Save it in your database
    Ok(StatsResponse::Stats(Json(Stats {
        status: "success".to_string(),
    })))
}
