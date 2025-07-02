mod cli;
mod models;
mod todo_manager;

use clap::Parser;
use cli::{Cli, run_cli};
use todo_manager::TodoManager;

fn main() {
    let cli = Cli::parse();

    // Initialize TodoManager with persistence - fail fast on errors
    let mut todo_manager = match TodoManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("❌ Failed to initialize todo manager: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = run_cli(cli, &mut todo_manager) {
        eprintln!("❌ Error: {e}");
        std::process::exit(1);
    }
}
