pub mod config;
pub mod usecases;
pub mod ports;
pub mod adapters;

use clap::Parser;
use std::path::PathBuf;
use usecases::scan::scan_directory;
use usecases::rename_files::rename_files;
use config::config::AppConfig;
use adapters::FsAdapter;

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

fn main() {
    let cli = Cli::parse();

    // Infrastructure - create adapters
    let fs = FsAdapter::new();

    // Load configuration
    let config = AppConfig::load(&cli.config).unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config: {}", e);
        AppConfig::default_config()
    });

    let path = PathBuf::from(&cli.directory);

    // Scan directory
    match scan_directory(&fs, &path, cli.recursive) {
        Ok(files) => {
            // Business logic - rename files
            let result = rename_files(&fs, &files, &config, cli.verbose, cli.fake);

            println!("\nSummary: {} renamed, {} unchanged", result.changed, result.unchanged);
            
            if cli.fake {
                println!("(fake mode - no actual changes made)");
            }
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
        }
    }
}
