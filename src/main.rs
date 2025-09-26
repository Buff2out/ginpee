use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Output file (default: project.md)
    #[arg(short, long, default_value = "project.md")]
    output: PathBuf,

    /// Top text to include in project.md
    #[arg(long)]
    top: Option<String>,

    /// Bottom text to include in project.md
    #[arg(long)]
    down: Option<String>,

    /// Files to include (e.g., "*.rs" "README.md")
    #[arg(long)]
    files: Vec<String>,

    /// Path to config file (default: geenpee.toml)
    #[arg(long, default_value = "geenpee.toml")]
    config: PathBuf,
}

#[derive(Deserialize, Default)]
struct Config {
    top: Option<Content>,
    down: Option<Content>,
    files: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Content {
    text: String,
}

fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = load_config(&cli.config).unwrap_or_default();

    let top = cli.top.or_else(|| config.top.map(|c| c.text));
    let down = cli.down.or_else(|| config.down.map(|c| c.text));
    let files = if !cli.files.is_empty() {
        cli.files
    } else {
        config.files.unwrap_or_default()
    };

    // TODO: Collect files based on filters
    // TODO: Build tree and content
    // TODO: Write to output file

    Ok(())
}