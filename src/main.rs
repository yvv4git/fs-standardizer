pub mod config;
pub mod usecases;
pub mod ports;
pub mod adapters;

use clap::Parser;
use std::path::PathBuf;
use usecases::scan::scan_directory;
use config::config::{AppConfig, RenameRule};
use ports::FileSystem;
use adapters::FsAdapter;
use regex::Regex;

#[derive(Parser)]
#[command(name = "fs-standardizer")]
#[command(about = "File system scanner and renamer", long_about = None)]
struct Cli {
    /// Directory to scan (default: current directory)
    #[arg(default_value = ".")]
    directory: String,

    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Scan recursively
    #[arg(short, long)]
    recursive: bool,

    /// Show old_name -> new_name for each file
    #[arg(short, long)]
    verbose: bool,

    /// Preview changes without renaming
    #[arg(short, long)]
    fake: bool,
}

fn apply_rules(filename: &str, rules: &[RenameRule]) -> String {
    let mut result = filename.to_string();
    
    for rule in rules {
        if let Ok(re) = Regex::new(&rule.pattern) {
            result = re.replace_all(&result, rule.replacement.as_str()).to_string();
        }
    }
    
    result
}

fn main() {
    let cli = Cli::parse();

    let fs = FsAdapter::new();

    let config = AppConfig::load(&cli.config).unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config: {}", e);
        AppConfig::default_config()
    });

    let path = PathBuf::from(&cli.directory);

    match scan_directory(&fs, &path, cli.recursive) {
        Ok(files) => {
            let mut changed_count = 0;
            let mut unchanged_count = 0;

            for file_path in &files {
                let file_name = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                let new_name = apply_rules(file_name, &config.rules);
                
                if new_name != file_name {
                    changed_count += 1;
                    
                    if cli.verbose {
                        println!("[RENAMED] {} -> {}", file_name, new_name);
                    }
                    
                    if !cli.fake {
                        let new_path = file_path.parent().unwrap().join(&new_name);
                        if let Err(e) = fs.rename(file_path, &new_path) {
                            eprintln!("Error renaming {:?}: {}", file_path, e);
                        }
                    }
                } else {
                    unchanged_count += 1;
                    if cli.verbose {
                        println!("[UNCHANGED] {}", file_name);
                    }
                }
            }

            println!("\nSummary: {} renamed, {} unchanged", changed_count, unchanged_count);
            
            if cli.fake {
                println!("(fake mode - no actual changes made)");
            }
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
        }
    }
}
