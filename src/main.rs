#![allow(unused)]
use std::collections::{HashMap, VecDeque};
use std::format;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

mod api;
mod app_config;
mod app_state;
mod bot;
mod logging;
mod mojang;

use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use azalea::prelude::*;
use azalea_viaversion::ViaVersionPlugin;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::api::handler::hello::hello_handler;
use crate::api::handler::login::login_handler;
use crate::api::middleware;
use crate::app_config::config;
use crate::app_state::AppState;
use crate::bot::activity::Activity;
use crate::bot::bot_controller::{BotController, SharedState};
use crate::bot::handle_event::handle_event;
use crate::logging::init_tracing;
use crate::mojang::change_skin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing(config().rust_log.as_deref().unwrap_or("info").to_string());
    tracing::info!("Starting ...");

    let addr = format!("0.0.0.0:{}", config().port.as_deref().unwrap_or("80"));

    let bot = BotController::new(SharedState::new())
        .with_activity(VecDeque::from([Activity::Say {
            message: "/tc Activity Hello World".to_string(),
        }]))
        .await
        .with_startup_commands(vec!["/pvp on".to_string()])
        .await
        .with_auto_respawn("/pw head".to_string())
        .await;

    let state = Arc::new(RwLock::new(AppState {
        bot_task: bot,
        sessions: HashMap::new(),
        secret: "SECRET".to_owned(),
    }));

    let app = Router::new()
        .route("/login", post(login_handler))
        .route(
            "/hello",
            get(hello_handler).layer(from_fn_with_state(
                state.clone(),
                middleware::auth::auth_middleware,
            )),
        )
        .with_state(state);

    tracing::info!("Listening on {}", &addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
