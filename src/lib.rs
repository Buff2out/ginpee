pub mod config;
pub mod collector;
pub mod formatter;
pub mod init;
pub mod writer;

pub use clap::{Subcommand, Parser};

#[derive(Subcommand)]
pub enum Commands {
    /// Generate ginpee.toml and .gpskip
    Init {
        /// Force overwrite existing files
        #[arg(long)]
        force: bool,
    },
    /// Generate project.md
    Run {
        /// Output file (default: project.md)
        #[arg(short, long, default_value = "project.md")]
        output: std::path::PathBuf,

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
        config: std::path::PathBuf,
    },
}

pub fn run_command(
    output: std::path::PathBuf,
    top: Option<String>,
    down: Option<String>,
    files: Vec<String>,
    config_path: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load(&config_path).unwrap_or_default();

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
    Ok(())
}