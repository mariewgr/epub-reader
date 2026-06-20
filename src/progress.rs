use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Progress {
    pub chapter: usize,
    pub scroll: usize,
}

fn progress_path(title: &str) -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".epub-reader");
    path.push(format!("{}.json", title.replace("/", "-")));
    path
}

pub fn load(title: &str) -> Option<Progress> {
    let path = progress_path(title);
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

pub fn save(title: &str, chapter: usize, scroll: usize) {
    let path = progress_path(title);
    fs::create_dir_all(path.parent().unwrap()).ok();
    let progress = Progress { chapter, scroll };
    if let Ok(json) = serde_json::to_string(&progress) {
        fs::write(path, json).ok();
    }
}
