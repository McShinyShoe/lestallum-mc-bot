use azalea::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::bot::{activity::Activity, bot_controller::SharedState};

#[derive(Default, Clone, Component)]
pub struct State {
    pub town_residents: Arc<Mutex<HashSet<String>>>,
    pub town_mayor: Arc<Mutex<String>>,
    pub town_assistants: Arc<Mutex<HashSet<String>>>,
    pub town_builders: Arc<Mutex<HashSet<String>>>,
    pub town_comayors: Arc<Mutex<HashSet<String>>>,
    pub town_helpers: Arc<Mutex<HashSet<String>>>,
    pub town_recruiters: Arc<Mutex<HashSet<String>>>,
    pub town_sheriffs: Arc<Mutex<HashSet<String>>>,
    pub town_vips: Arc<Mutex<HashSet<String>>>,
    pub town_trusteds: Arc<Mutex<HashSet<String>>>,
    pub on_towny: Arc<Mutex<bool>>,
    pub shutdown_signal_sent: Arc<Mutex<bool>>,
    pub shared_state: Arc<tokio::sync::Mutex<SharedState>>,
}
