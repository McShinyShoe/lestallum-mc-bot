use azalea::prelude::*;

use crate::bot::{
    app_state::State,
    events::{chat::handle_chat, death::handle_death, packet::handle_packet, spawn::handle_spawn},
};

pub async fn handle_event(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    tracing::debug!("Got event {:?}", event);
    match event {
        Event::Spawn => handle_spawn(&bot, &state).await?,
        Event::Packet(packet) => handle_packet(&bot, &state, packet).await?,
        Event::Chat(chat) => handle_chat(&bot, &state, chat).await?,
        Event::Death(combat_kill) => handle_death(&bot, &state, combat_kill).await?,
        _ => {}
    }

    Ok(())
}
