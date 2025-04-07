use std::path::PathBuf;
use tauri::ipc::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Permission denied when accessing {path}")]
    PermissionDenied { path: PathBuf },

    #[error("IO Error {0}")]
    Io(std::io::Error),

    #[error("Path {0} not found")]
    NotFound(PathBuf),

    #[error("Walkdir error: {0}")]
    WalkDirError(walkdir::Error),

    #[error("Path {0} is not a directory")]
    NotADirectory(PathBuf),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Path resolution error: {0}")]
    PathResolution(String),

    #[error("No vault selected")]
    NoVaultSelected,

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Unknown error occurred")]
    Unknown,
}

impl<T> From<std::sync::PoisonError<T>> for VaultError {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        VaultError::LockError(error.to_string())
    }
}

impl From<VaultError> for InvokeError {
    fn from(error: VaultError) -> Self {
        InvokeError::from(error.to_string())
    }
}

impl From<walkdir::Error> for VaultError {
    fn from(err: walkdir::Error) -> Self {
        VaultError::WalkDirError(err)
    }
}

impl From<std::io::Error> for VaultError {
    fn from(err: std::io::Error) -> Self {
        VaultError::Io(err)
    }
}
