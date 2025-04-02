use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Path resolution error: {0}")]
    PathResolution(String),
    
    #[error("No vault selected")]
    NoVaultSelected,
    
    #[error("Lock error: {0}")]
    LockError(String),
}

impl<T> From<std::sync::PoisonError<T>> for VaultError {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        VaultError::LockError(error.to_string())
    }
}