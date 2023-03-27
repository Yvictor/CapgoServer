use poem::{listener::TcpListener, Result, Route};
use poem_openapi::{param::Query, payload::Json, payload::PlainText, OpenApi, OpenApiService};
use std::env;
use tracing::info;
use tracing_subscriber;
use CapgoServer::api::live_update::{get_update_info, AppInfos, ErrorResponse, UpdateResponse};
use CapgoServer::api::stats::{handle_stats_request, AppStats, StatsResponse};
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
        let update_info = get_update_info(&app_infos).await;

        match update_info {
            Some(info) => Ok(UpdateResponse::UpdateInfo(Json(info))),
            None => Ok(UpdateResponse::Error(Json(ErrorResponse {
                message: "Error version not found".to_string(),
                error: "No update available".to_string(),
            }))),
        }
    }
    #[oai(path = "/stats", method = "post")]
    async fn stats_request(&self, app_stats: Json<AppStats>) -> Result<StatsResponse> {
        handle_stats_request(&app_stats).await
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3333".to_string())
        .parse::<u16>()
        .expect("Invalid PORT value");
    let domain =
        env::var("DOMAIN").unwrap_or_else(|_| format!("http://127.0.0.1:{}", port).to_string());

    let api_service =
        OpenApiService::new(Api, "CapgoServer", "1.0").server(&format!("{}/api", domain));
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    poem::Server::new(TcpListener::bind((host, port)))
        .run(app)
        .await
}
