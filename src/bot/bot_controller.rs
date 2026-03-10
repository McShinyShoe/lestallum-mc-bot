use azalea::auto_respawn;
use azalea_viaversion::ViaVersionPlugin;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::app_config::config;
use crate::bot::bot_state::State;
use crate::bot::{activity::Activity, handle_event::handle_event};
use crate::mojang::change_skin;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::LocalSet;

#[derive(Debug)]
pub struct BotController {
    pub bot_task: Mutex<Option<std::thread::JoinHandle<anyhow::Result<()>>>>,
    pub shared_state: Arc<tokio::sync::Mutex<SharedState>>,
}

#[derive(Default, Debug)]
pub struct SharedState {
    pub shutdown_signal: bool,
    pub activity_list: VecDeque<Activity>,
    pub startup_commands: Vec<String>,
    pub auto_respawn: Option<String>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            shutdown_signal: false,
            activity_list: VecDeque::new(),
            startup_commands: Vec::new(),
            auto_respawn: None,
        }
    }

    pub fn set_activity(&mut self, activity_list: VecDeque<Activity>) {
        self.activity_list = activity_list;
    }

    pub fn set_startup_commands(&mut self, startup_commands: Vec<String>) {
        self.startup_commands = startup_commands;
    }

    pub fn set_auto_respawn(&mut self, auto_respawn: String) {
        self.auto_respawn = Some(auto_respawn);
    }

    pub fn with_activity(mut self, activity_list: VecDeque<Activity>) -> Self {
        self.set_activity(activity_list);
        self
    }

    pub fn with_startup_commands(mut self, startup_commands: Vec<String>) -> Self {
        self.set_startup_commands(startup_commands);
        self
    }

    pub fn with_auto_respawn(mut self, auto_respawn: String) -> Self {
        self.set_auto_respawn(auto_respawn);
        self
    }
}

use azalea::prelude::*;

impl BotController {
    pub fn new(shared_state: SharedState) -> Self {
        Self {
            bot_task: Mutex::new(None),
            shared_state: Arc::new(tokio::sync::Mutex::new(shared_state)),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        tracing::info!("Starting mc bot");
        let mut task = self.bot_task.lock().unwrap();

        if task.is_some() {
            tracing::info!("Bot already started");
            return Ok(());
        }

        let shared_state = self.shared_state.clone();

        let handle = thread::spawn(move || -> anyhow::Result<()> {
            let runtime = Builder::new_current_thread().enable_all().build()?;
            let local = LocalSet::new();

            runtime.block_on(local.run_until(run_bot(shared_state)))?;
            Ok(())
        });
        *task = Some(handle);
        tracing::info!("Bot started.");
        Ok(())
    }
    pub async fn join(&self) -> anyhow::Result<()> {
        let mut task = self.bot_task.lock().unwrap();

        if task.is_none() {
            tracing::info!("Bot already ended");
            return Ok(());
        }

        if let Some(handle) = task.take() {
            handle.join().unwrap()?;
        }

        Ok(())
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        {
            let mut a = self.shared_state.lock().await;
            a.shutdown_signal = true;
        }
        self.join().await?;

        Ok(())
    }

    pub async fn add_activity(&self, activity: Activity) -> anyhow::Result<()> {
        {
            let mut shared_state = self.shared_state.lock().await;
            shared_state.activity_list.push_back(activity);
        }
        Ok(())
    }

    pub async fn with_activity(mut self, activity_list: VecDeque<Activity>) -> Self {
        {
            let mut state = self.shared_state.lock().await;
            state.set_activity(activity_list);
        }
        self
    }

    pub async fn with_startup_commands(mut self, startup_commands: Vec<String>) -> Self {
        {
            let mut state = self.shared_state.lock().await;
            state.set_startup_commands(startup_commands);
        }
        self
    }

    pub async fn with_auto_respawn(mut self, auto_respawn: String) -> Self {
        {
            let mut state = self.shared_state.lock().await;
            state.set_auto_respawn(auto_respawn);
        }
        self
    }
}

async fn run_bot(shared_state: Arc<tokio::sync::Mutex<SharedState>>) -> anyhow::Result<()> {
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

    let state = State {
        shared_state: shared_state,
        ..Default::default()
    };

    tracing::info!("Initializing complete...");

    ClientBuilder::new()
        .add_plugins(ViaVersionPlugin::start(&cfg.mc_version).await)
        .set_handler(handle_event)
        .set_state(state.clone())
        .reconnect_after(None)
        .start(account, format!("{}:{}", cfg.server_uri, cfg.server_port))
        .await;

    Ok(())
}
