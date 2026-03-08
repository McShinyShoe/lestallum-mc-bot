use std::sync::Arc;

use anyhow::Ok;
use azalea::{prelude::*, protocol::packets::game::ClientboundPlayerCombatKill};

use crate::bot::bot_state::State;

pub async fn handle_death(
    bot: &Client,
    state: &State,
    combat_kill: Option<Arc<ClientboundPlayerCombatKill>>,
) -> anyhow::Result<()> {
    bot.chat("/pw head");
    Ok(())
}
