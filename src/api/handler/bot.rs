use axum::{Json, extract::State};
use serde::Deserialize;

use crate::{
    api::api_response::{ApiResponse, JsonApiResponse},
    app_state::AppStateStore,
    bot::activity::Activity,
};

pub async fn bot_start_handler(State(state): State<AppStateStore>) -> JsonApiResponse {
    let state = state.write().await;
    state.bot_task.start().await;
    Json(ApiResponse::ok("Bot started."))
}

pub async fn bot_stop_handler(State(state): State<AppStateStore>) -> JsonApiResponse {
    let state = state.write().await;
    state.bot_task.stop().await;
    Json(ApiResponse::ok("Bot stopped."))
}

#[derive(Deserialize)]
pub struct BotSayInput {
    message: String,
}

pub async fn bot_say_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<BotSayInput>,
) -> JsonApiResponse {
    let state = state.write().await;
    state
        .bot_task
        .add_activity(Activity::Say {
            message: body.message.clone(),
        })
        .await;
    Json(ApiResponse::ok(format!("Saying \"{}\"", body.message)))
}
