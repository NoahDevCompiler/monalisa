use crate::errors::VaultError;
use crate::state::SharedState;
use std::fs;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

pub struct VaultManager {
    state: SharedState,
}

impl VaultManager {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
    pub fn init_default(&self, app_handle: &AppHandle) -> Result<PathBuf, VaultError> {
        let vault_path = app_handle
            .path()
            .resolve("Monalisa Vault", BaseDirectory::Document)
            .map_err(|e| VaultError::PathResolution(e.to_string()))?;

        fs::create_dir_all(&vault_path)?;

        self.set_current_vault(vault_path.clone())?;
        Ok(vault_path)
    }
    pub fn set_current_vault(&self, path: PathBuf) -> Result<(), VaultError> {
        let mut state = self.state.lock()?;
        state.current_vault = Some(path);
        Ok(())
    }
    pub fn get_current_vault(&self) -> Result<PathBuf, VaultError> {
        let state = self.state.lock()?;

        state.current_vault
        .clone()
        .ok_or(VaultError::NoVaultSelected)

    }
    //pub fn create_vault(???)-> ??? {}
}
