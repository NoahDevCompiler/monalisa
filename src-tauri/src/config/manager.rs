use serde::Serialize;
use std::path::{Path, PathBuf};
pub struct AppConfig {
    pub vaults: Vec<PathBuf>,
    pub last_active_vault: Option<PathBuf>
}





