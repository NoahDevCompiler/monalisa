use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub children: Option<Vec<DirEntry>>,
}

#[derive(Serialize, Clone)]
pub struct DirContent {
    pub entries: Vec<DirEntry>,
}