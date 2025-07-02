mod cli;
mod models;
mod todo_manager;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, run_cli};
use todo_manager::TodoManager;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize TodoManager with persistence - fail fast on errors
    let mut todo_manager = TodoManager::new()?;

    run_cli(cli, &mut todo_manager)?;
    Ok(())
}
