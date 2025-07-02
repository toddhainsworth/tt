use clap::{Parser, Subcommand};
use crate::todo_manager::TodoManager;

#[derive(Parser)]
#[command(name = "tt")]
#[command(about = "A simple Todo CLI application")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
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
        Some(command) => match command {
            Commands::Add { title } => {
                let todo = todo_manager.add_todo(title)?;
                println!("✅ Added todo: {}", todo.title);
                Ok(())
            }
            Commands::List => {
                display_todos(todo_manager);
                Ok(())
            }
            Commands::Complete { id } => {
                todo_manager.mark_completed(id)?;
                if let Some(todo) = todo_manager.get_todo(id) {
                    println!("✅ Marked as completed: {}", todo.title);
                }
                Ok(())
            }
            Commands::Incomplete { id } => {
                todo_manager.mark_incomplete(id)?;
                if let Some(todo) = todo_manager.get_todo(id) {
                    println!("⏳ Marked as incomplete: {}", todo.title);
                }
                Ok(())
            }
            Commands::Toggle { id } => {
                todo_manager.toggle_completed(id)?;
                if let Some(todo) = todo_manager.get_todo(id) {
                    let status = if todo.completed { "✅ completed" } else { "⏳ incomplete" };
                    println!("🔄 Toggled: {} is now {}", todo.title, status);
                }
                Ok(())
            }
            Commands::Delete { id } => {
                todo_manager.delete_todo(id)?;
                println!("🗑️  Todo deleted successfully");
                Ok(())
            }
        },
        None => {
            // Default behavior: list todos
            display_todos(todo_manager);
            Ok(())
        }
    }
}

fn display_todos(todo_manager: &TodoManager) {
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
} 