use std::fs;
use std::path::PathBuf;
use crate::ports::FileSystem;

pub struct FsAdapter;

impl FsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for FsAdapter {
    fn scan(&self, path: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>, String> {
        let mut files = Vec::new();
        
        if !path.exists() {
            return Err(format!("Path does not exist: {:?}", path));
        }

        if path.is_file() {
            return Ok(vec![path.clone()]);
        }

        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        for entry in entries.flatten() {
            let entry_path = entry.path();
            
            if entry_path.is_file() {
                files.push(entry_path);
            } else if entry_path.is_dir() && recursive {
                let sub_files = self.scan(&entry_path, true)?;
                files.extend(sub_files);
            }
        }

        Ok(files)
    }

    fn rename(&self, old_path: &PathBuf, new_path: &PathBuf) -> Result<(), String> {
        fs::rename(old_path, new_path)
            .map_err(|e| format!("Failed to rename file: {}", e))
    }
}
