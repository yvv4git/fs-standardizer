use std::path::PathBuf;
use crate::ports::FileSystem;
use crate::config::config::{AppConfig, RenameRule};
use super::transliterate::transliterate;
use std::path::Path;
use regex::Regex;

pub fn apply_rules(filename: &str, rules: &[RenameRule]) -> String {
    // Split filename and extension
    let path = Path::new(filename);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    
    // Apply regex rules to the stem (filename without extension)
    let mut result = stem.to_string();
    
    for rule in rules {
        if let Ok(re) = Regex::new(&rule.pattern) {
            result = re.replace_all(&result, rule.replacement.as_str()).to_string();
        }
    }
    
    // Transliterate non-ASCII characters
    result = transliterate(&result);
    
    // Reconstruct filename with extension
    if ext.is_empty() {
        result
    } else {
        format!("{}.{}", result, ext)
    }
}

pub struct RenameResult {
    pub changed: usize,
    pub unchanged: usize,
}

pub fn rename_files<F: FileSystem>(
    fs: &F,
    files: &[PathBuf],
    config: &AppConfig,
    verbose: bool,
    fake: bool,
) -> RenameResult {
    let mut changed_count = 0;
    let mut unchanged_count = 0;

    for file_path in files {
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Skip hidden files (starting with dot)
        if file_name.starts_with('.') {
            continue;
        }

        let new_name = apply_rules(file_name, &config.rules);
        
        if new_name.is_empty() || new_name == file_name {
            if new_name.is_empty() {
                unchanged_count += 1;
            } else {
                unchanged_count += 1;
            }
            
            if verbose && new_name != file_name {
                println!("[UNCHANGED] {}", file_name);
            }
        } else {
            changed_count += 1;
            
            if verbose {
                println!("[RENAMED] {} -> {}", file_name, new_name);
            }
            
            if !fake {
                let new_path = file_path.parent().unwrap().join(&new_name);
                if let Err(e) = fs.rename(file_path, &new_path) {
                    eprintln!("Error renaming {:?}: {}", file_path, e);
                }
            }
        }
    }

    RenameResult {
        changed: changed_count,
        unchanged: unchanged_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_rules(patterns: Vec<(&str, &str)>) -> Vec<RenameRule> {
        patterns.into_iter()
            .map(|(pattern, replacement)| RenameRule {
                pattern: pattern.to_string(),
                replacement: replacement.to_string(),
            })
            .collect()
    }

    #[test]
    fn test_apply_rules_simple() {
        let rules = create_test_rules(vec!(
            ("-", "_"),
        ));
        
        let result = apply_rules("test-file.txt", &rules);
        assert_eq!(result, "test_file.txt");
    }

    #[test]
    fn test_apply_rules_with_extension() {
        let rules = create_test_rules(vec!(
            ("-", "_"),
            (" ", "_"),
        ));
        
        let result = apply_rules("hello world.mp3", &rules);
        assert_eq!(result, "hello_world.mp3");
    }

    #[test]
    fn test_apply_rules_cyrillic() {
        let rules = create_test_rules(vec!(
            ("-", "_"),
            (" ", "_"),
        ));
        
        let result = apply_rules("привет мир.txt", &rules);
        assert_eq!(result, "privet_mir.txt");
    }

    #[test]
    fn test_apply_rules_mixed() {
        let rules = create_test_rules(vec!(
            ("-", "_"),
            (" ", "_"),
            ("__+", "_"),
        ));
        
        let result = apply_rules("avto_Kolya Funk - Привет.txt", &rules);
        assert_eq!(result, "avto_Kolya_Funk_privet.txt");
    }

    #[test]
    fn test_apply_rules_keeps_extension() {
        let rules = create_test_rules(vec!(
            (" ", "_"),
        ));
        
        let result = apply_rules("my file.mp3", &rules);
        assert!(result.ends_with(".mp3"));
    }
}
