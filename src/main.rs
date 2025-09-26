//! `ginpee` — утилита для генерации `project.md` с деревом файлов и их содержимым.
//!
//! ## Примеры:
//!
//! ```bash
//! ginpee init
//! ginpee run --output docs.md
//! ```
use ginpee::{Commands, run_command};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            ginpee::init::run(force)?;
        }
        Commands::Run { output, top, down, files, config } => {
            run_command(output, top, down, files, config)?;
        }
    }

    Ok(())
}