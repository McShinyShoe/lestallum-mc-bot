use anyhow::Ok;
use azalea::prelude::*;

use crate::app_state::State;

pub async fn handle_spawn(bot: &Client, state: &State) -> anyhow::Result<()> {
    tracing::info!("Spawning");
    if !*state.on_towny.lock().await {
        tracing::info!("Connecting to towny");
        bot.chat("/server towny");
    }
    Ok(())
}
