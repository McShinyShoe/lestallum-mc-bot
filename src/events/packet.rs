use std::sync::Arc;

use azalea::{
    prelude::*,
    protocol::packets::{game::ClientboundGamePacket},
};

use crate::app_state::State;

pub async fn handle_packet(
    bot: &Client,
    state: &State,
    packet: Arc<ClientboundGamePacket>,
) -> anyhow::Result<()> {
    Ok(())
}
