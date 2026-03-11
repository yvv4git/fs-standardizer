use std::path::PathBuf;
use crate::ports::FileSystem;

pub fn rename_file<F: FileSystem>(
    fs: &F,
    old_path: &PathBuf,
    new_name: &str,
) -> Result<(), String> {
    let parent = old_path.parent().ok_or("No parent directory")?;
    let new_path = parent.join(new_name);
    fs.rename(old_path, &new_path)
}
