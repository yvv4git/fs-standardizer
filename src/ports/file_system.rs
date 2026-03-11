use std::path::PathBuf;

pub trait FileSystem {
    fn scan(&self, path: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>, String>;
    fn rename(&self, old_path: &PathBuf, new_path: &PathBuf) -> Result<(), String>;
}
