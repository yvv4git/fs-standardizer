use std::path::PathBuf;
use crate::ports::FileSystem;

pub fn scan_directory<F: FileSystem>(
    fs: &F,
    path: &PathBuf,
    recursive: bool,
) -> Result<Vec<PathBuf>, String> {
    fs.scan(path, recursive)
}
