use std::format;
use std::path::PathBuf;

mod app_config;
mod bot;

use azalea::prelude::*;
use azalea_viaversion::ViaVersionPlugin;

mod logging;

use crate::app_config::{config};
use crate::bot::handle_event::handle_event;
use crate::logging::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing(config().rust_log.as_deref().unwrap_or("info").to_string());

    let email = &config().email;
    let mc_version = &config().mc_version;

    let cache_file = PathBuf::from(config().auth_cache_file.as_deref().unwrap_or("info").to_string());
    azalea_auth::auth(
        &email,
        azalea_auth::AuthOpts {
            cache_file: Some(cache_file),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let account = Account::microsoft(&email).await?;
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
