use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub current_vault: Option<PathBuf>,
}

pub type SharedState = Arc<Mutex<AppState>>;

pub fn init_state() -> SharedState {
    Arc::new(Mutex::new(AppState::default()))
}