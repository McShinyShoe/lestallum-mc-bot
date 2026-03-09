#![allow(unused)]
use std::collections::VecDeque;
use std::format;
use std::path::PathBuf;
use std::time::Duration;

mod app_config;
mod bot;
mod logging;
mod mojang;

use azalea::prelude::*;
use azalea_viaversion::ViaVersionPlugin;
use tokio::time::sleep;

use crate::app_config::config;
use crate::bot::activity::Activity;
use crate::bot::bot_controller::{BotController, SharedState};
use crate::bot::handle_event::handle_event;
use crate::logging::init_tracing;
use crate::mojang::change_skin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing(config().rust_log.as_deref().unwrap_or("info").to_string());
    tracing::info!("Starting ...");
    let bot = BotController::new(SharedState::new())
        .with_activity(VecDeque::from([Activity::Say {
            message: "/tc Activity Hello World".to_string(),
        }]))
        .await
        .with_startup_commands(vec![
            "/tc Startup Commands".to_string(),
            "/pvp on".to_string(),
        ])
        .await
        .with_auto_respawn("/pw head".to_string())
        .await;
    bot.start().await?;
    sleep(Duration::from_secs(30)).await;
    bot.stop().await?;
    tracing::info!("Bot ended");
    Ok(())
}
