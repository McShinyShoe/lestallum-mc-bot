use axum::{Json, extract::State};

use crate::{
    api::api_response::{ApiResponse, JsonApiResponse},
    app_state::AppStateStore,
};

pub async fn hello_handler(State(_): State<AppStateStore>) -> JsonApiResponse {
    Json(ApiResponse::ok("Hello World."))
}
