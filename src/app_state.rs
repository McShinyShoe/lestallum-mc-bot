use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{Mutex, RwLock}, task::JoinHandle};

#[derive(Debug)]
pub struct AppState {
    pub bot_task: Mutex<Option<JoinHandle<()>>>,
}
pub type AppStateStore = Arc<AppState>;