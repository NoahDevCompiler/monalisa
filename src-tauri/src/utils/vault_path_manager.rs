use::std::path::PathBuf;
use std::sync::Mutex;

static VAULT_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

//pub fn set_vault_path(path: String) -> Result<(), String> {
    //let path_buf = PathBuf::from(path);

    //let vault_path = VAULT_PATH.lock().map_err(|e| format!("Fail"))
//}