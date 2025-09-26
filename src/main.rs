mod config;
mod collector;
mod formatter;
mod writer;

use clap::Parser;
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

    /// Path to config file (default: ginpee.toml)
    #[arg(long, default_value = "ginpee.toml")]
    config: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = config::load(&cli.config).unwrap_or_default();

    let top = cli.top.or_else(|| config.top.map(|c| c.text));
    let down = cli.down.or_else(|| config.down.map(|c| c.text));
    let files = if !cli.files.is_empty() {
        cli.files
    } else {
        config.files.map(|f| f.include).unwrap_or_default()
    };

    let base_path = std::env::current_dir()?;
    let collected_files = collector::collect_files(&base_path, &files, ".gpskip")?;

    let (tree, contents) = formatter::build_tree_and_content(&collected_files, &base_path)?;

    writer::write_project_md(&cli.output, top, &tree, &contents, down)?;

    Ok(())
}