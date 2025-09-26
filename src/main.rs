mod config;
mod collector;
mod formatter;
mod writer;
mod init;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate ginpee.toml and .gpskip
    Init {
        /// Force overwrite existing files
        #[arg(long)]
        force: bool,
    },
    /// Generate project.md
    Generate {
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

        /// Path to config file (default: ginpee.toml)
        #[arg(long, default_value = "ginpee.toml")]
        config: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            init::run(force)?;
        }
        Commands::Generate { output, top, down, files, config } => {
            let config = config::load(&config).unwrap_or_default();

            let top = top.or_else(|| config.top.map(|c| c.text));
            let down = down.or_else(|| config.down.map(|c| c.text));
            let files = if !files.is_empty() {
                files
            } else {
                config.files.map(|f| f.include).unwrap_or_default()
            };

            let base_path = std::env::current_dir()?;
            let collected_files = collector::collect_files(&base_path, &files, ".gpskip")?;

            let (tree, contents) = formatter::build_tree_and_content(&collected_files, &base_path)?;

            writer::write_project_md(&output, top, &tree, &contents, down)?;
        }
    }

    Ok(())
}