use std::format;

mod app_config;
mod app_state;
mod handle_event;

use azalea::prelude::*;
use azalea_viaversion::ViaVersionPlugin;

mod events;
mod logging;

use crate::app_config::{config};
use crate::handle_event::handle_event;
use crate::logging::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing(config().rust_log.as_deref().unwrap_or("info").to_string());

    let email = &config().email;
    let mc_version = &config().mc_version;

    let account = Account::microsoft(&email);
    let account = account.await?;
    ClientBuilder::new()
        .add_plugins(ViaVersionPlugin::start(mc_version).await)
        .set_handler(handle_event)
        .start(
            account,
            format!("{}:{}", &config().server_uri, &config().server_port),
        )
        .await;
    Ok(())
}
