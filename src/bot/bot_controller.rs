use azalea::auto_respawn;
use azalea_viaversion::ViaVersionPlugin;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::app_config::config;
use crate::bot::{handle_event::handle_event, activity::Activity};
use crate::mojang::change_skin;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::LocalSet;

pub struct BotController {
    pub bot_task: Mutex<Option<std::thread::JoinHandle<anyhow::Result<()>>>>,
    pub shutdown_tx: Mutex<Option<watch::Sender<bool>>>,
    pub activity_list: Arc<Mutex<VecDeque<Activity>>>,
    pub startup_commands: Arc<Mutex<Vec<String>>>,
    pub auto_respawn: Arc<Mutex<Option<String>>>,
}

use azalea::prelude::*;

impl BotController {
    pub fn new() -> Self {
        Self {
            bot_task: Mutex::new(None),
            shutdown_tx: Mutex::new(None),
            activity_list: Arc::new(Mutex::new(VecDeque::new())),
            startup_commands: Arc::new(Mutex::new(Vec::new())),
            auto_respawn: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_activity(mut self, activity_list: VecDeque<Activity>) -> Self {
        self.activity_list = Arc::new(Mutex::new(activity_list));
        self
    }

    pub fn with_startup_commands(mut self, startup_commands: Vec<String>) -> Self {
        self.startup_commands = Arc::new(Mutex::new(startup_commands));
        self
    }

    pub fn with_auto_respawn(mut self, auto_respawn: String) -> Self {
        self.auto_respawn = Arc::new(Mutex::new(Some(auto_respawn)));
        self
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        tracing::info!("Starting mc bot");
        let mut task = self.bot_task.lock().unwrap();

        if task.is_some() {
            tracing::info!("Bot already started");
            return Ok(());
        }

        let (tx, mut rx) = tokio::sync::watch::channel(false);
        *self.shutdown_tx.lock().unwrap() = Some(tx);

        let handle = thread::spawn(move || -> anyhow::Result<()> {
            let runtime = Builder::new_current_thread().enable_all().build()?;

            let local = LocalSet::new();

            runtime.block_on(local.run_until(async move {
                tokio::select! {
                    res = run_bot() => res,
                    _ = rx.changed() => {
                        tracing::info!("Shutdown signal received");
                        Ok(())
                    }
                }
            }))
        });
        *task = Some(handle);
        tracing::info!("Bot started.");
        Ok(())
    }
    pub async fn stop(&self) -> anyhow::Result<()> {
        if let Some(tx) = self.shutdown_tx.lock().unwrap().take() {
            let _ = tx.send(true);
        }

        if let Some(handle) = self.bot_task.lock().unwrap().take() {
            handle.join().unwrap()?;
        }

        Ok(())
    }
}

async fn run_bot() -> anyhow::Result<()> {
    tracing::info!("Initializing bot...");
    let cfg = config();

    let cache_file = PathBuf::from(cfg.auth_cache_file.as_deref().unwrap_or("info"));

    azalea_auth::auth(
        &cfg.email,
        azalea_auth::AuthOpts {
            cache_file: Some(cache_file),
            ..Default::default()
        },
    )
    .await?;

    let account = Account::microsoft(&cfg.email).await?;

    if let Some(token) = account.access_token() {
        change_skin(&token, "./me.png").await?;
    }

    tracing::info!("Initializing complete...");

    ClientBuilder::new()
        .add_plugins(ViaVersionPlugin::start(&cfg.mc_version).await)
        .set_handler(handle_event)
        .start(account, format!("{}:{}", cfg.server_uri, cfg.server_port))
        .await;

    Ok(())
}
