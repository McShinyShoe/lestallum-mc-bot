use std::time::Duration;

use azalea::prelude::*;
use tokio::time::sleep;

use crate::bot::{
    bot_state::State,
    events::{chat::handle_chat, death::handle_death, packet::handle_packet, spawn::handle_spawn},
};

pub async fn handle_event(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    {
        let st = state.shared_state.lock().await;
        let mut st_sent = state.shutdown_signal_sent.lock().await;
        if st.shutdown_signal && !*st_sent {
            tracing::info!("Got shutdown signal!");
            sleep(Duration::from_millis(500)).await;
            bot.disconnect();
            *st_sent = true;
            return Ok(());
        }
    }

    match event {
        Event::Spawn => handle_spawn(&bot, &state).await?,
        Event::Packet(packet) => handle_packet(&bot, &state, packet).await?,
        Event::Chat(chat) => handle_chat(&bot, &state, chat).await?,
        Event::Death(combat_kill) => handle_death(&bot, &state, combat_kill).await?,
        _ => {}
    }

    Ok(())
}
