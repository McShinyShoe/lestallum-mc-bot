#![allow(unused)]
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
use crate::bot::bot_controller::BotController;
use crate::bot::handle_event::handle_event;
use crate::logging::init_tracing;
use crate::mojang::change_skin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing(config().rust_log.as_deref().unwrap_or("info").to_string());
    tracing::info!("Starting ...");
    let bot = BotController::new();
    bot.start().await?;
    sleep(Duration::from_secs(90)).await;
    bot.stop().await?;
    Ok(())
}
