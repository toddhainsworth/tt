mod models;
mod todo_manager;
mod cli;

use clap::Parser;
use cli::{Cli, run_cli};
use todo_manager::TodoManager;

fn main() {
    let cli = Cli::parse();
    let mut todo_manager = TodoManager::new();

    if let Err(e) = run_cli(cli, &mut todo_manager) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}
