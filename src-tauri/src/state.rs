#![allow(dead_code)]
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    current_vault: Mutex<String>
}

pub type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub fn new() -> Self {
        Self {
            current_vault: SharedState::new(Arc<Mutex<AppState>)
        }
    }
}