use clap::{Parser, Subcommand};
use crate::todo_manager::TodoManager;

#[derive(Parser)]
#[command(name = "tt")]
#[command(about = "A simple Todo CLI application")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo item
    Add {
        /// The title of the todo item
        title: String,
    },
    /// List all todo items
    List,
    /// Mark a todo item as completed
    Complete {
        /// The ID of the todo item to mark as completed
        id: usize,
    },
    /// Mark a todo item as incomplete
    Incomplete {
        /// The ID of the todo item to mark as incomplete
        id: usize,
    },
    /// Toggle a todo item's completed status
    Toggle {
        /// The ID of the todo item to toggle
        id: usize,
    },
    /// Delete a todo item
    Delete {
        /// The ID of the todo item to delete
        id: usize,
    },
}

pub fn run_cli(cli: Cli, todo_manager: &mut TodoManager) -> Result<(), String> {
    match cli.command {
        Commands::Add { title } => {
            let todo = todo_manager.add_todo(title);
            println!("✅ Added todo: {}", todo.title);
            Ok(())
        }
        Commands::List => {
            let todos = todo_manager.list_todos();
            if todos.is_empty() {
                println!("📝 No todos found. Add one with `tt add <title>`");
            } else {
                println!("📝 Your todos:");
                for (id, todo) in todos.iter().enumerate() {
                    let status = if todo.completed { "✅" } else { "⏳" };
                    println!("  {} [{}] {}", id, status, todo.title);
                }
            }
            Ok(())
        }
        Commands::Complete { id } => {
            match todo_manager.mark_completed(id) {
                Ok(()) => {
                    if let Some(todo) = todo_manager.get_todo(id) {
                        println!("✅ Marked as completed: {}", todo.title);
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::Incomplete { id } => {
            match todo_manager.mark_incomplete(id) {
                Ok(()) => {
                    if let Some(todo) = todo_manager.get_todo(id) {
                        println!("⏳ Marked as incomplete: {}", todo.title);
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::Toggle { id } => {
            match todo_manager.toggle_completed(id) {
                Ok(()) => {
                    if let Some(todo) = todo_manager.get_todo(id) {
                        let status = if todo.completed { "✅ completed" } else { "⏳ incomplete" };
                        println!("🔄 Toggled: {} is now {}", todo.title, status);
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::Delete { id } => {
            match todo_manager.delete_todo(id) {
                Ok(()) => {
                    println!("🗑️  Todo deleted successfully");
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    }
} 