use poem::{listener::TcpListener, Route, Result};
use poem_openapi::{
    param::Path, param::Query, payload::Json, payload::PlainText, OpenApi, OpenApiService, ApiResponse, Object
};
use serde::{Deserialize, Serialize};

mod update_info;
use update_info::UpdateInfo;

#[derive(Debug, Deserialize, Object)]
struct AppInfos {
    platform: String,
    device_id: String,
    app_id: String,
    custom_id: Option<String>,
    plugin_version: String,
    version_build: String,
    version_code: String,
    version_name: String,
    version_os: String,
    is_emulator: bool,
    is_prod: bool,
}

#[derive(Debug, Serialize, Object)]
struct ErrorResponse {
    error: String,
    // version: String,
    // url: String,
}

#[derive(ApiResponse)]
enum UpdateResponse {
    #[oai(status = 200)]
    UpdateInfo(Json<UpdateInfo>),

    #[oai(status = 404)]
    Error(Json<ErrorResponse>),
}

fn get_update_info(app_infos: &AppInfos) -> Option<UpdateInfo> {
    // Define the latest version for each platform
    // let latest_versions = vec![
    //     ("ios", "1.2.0"),
    //     ("android", "1.1.0"),
    // ];

    // let platform_latest_version = latest_versions.iter().find(|(p, _)| p == &app_infos.platform);
    let latest_version = "0.0.2";
    Some(UpdateInfo {
        version: latest_version.to_string(),
        url: "https://path_to_the_zip_file_of_the_code.com".to_string(),
    })
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
    #[oai(path = "/update", method = "post")]
    async fn update_request(
        &self,
        // _app_id: Path<String>,
        app_infos: Json<AppInfos>,
    ) -> Result<UpdateResponse> {
        let update_info = get_update_info(&app_infos);

    
        match update_info {
            Some(info) => Ok(UpdateResponse::UpdateInfo(Json(info))),
            None => Ok(UpdateResponse::Error(Json(ErrorResponse {
                error: "No update available".to_string(),
            }))),
            // None => Err(Json(ErrorResponse {
            //     error: "No update available".to_string(),
            // })),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
