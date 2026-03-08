use clap::Parser;

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
}

fn main() {
    let cli = Cli::parse();

    println!("Directory: {}", cli.directory);
    println!("Config:    {}", cli.config);
    println!("Recursive: {}", cli.recursive);
}
