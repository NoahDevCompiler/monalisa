use crate::config::manager::AppConfig;
use std::sync::Mutex;

pub struct ConfigState(pub Mutex<AppConfig>);
