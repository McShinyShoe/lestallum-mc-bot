use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{Mutex, RwLock}, task::JoinHandle};

use crate::{api::claims::Claims, bot::bot_controller::BotController};

#[derive(Debug)]
pub struct AppState {
    pub bot_task: BotController,
    pub sessions: HashMap<String, Claims>,
    pub secret: String,
}
pub type AppStateStore = Arc<RwLock<AppState>>;