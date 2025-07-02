use crate::todo_manager::TodoManager;
use clap::{Parser, Subcommand};
use colored::*;

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
        /// The priority of the todo item (1-4, 1 = highest, 4 = lowest)
        #[arg(short, long, value_name = "PRIORITY", default_value_t = 4)]
        priority: u8,
    },
    /// Edit an existing todo item
    Edit {
        /// The ID of the todo item to edit
        id: usize,
        /// The new title (optional)
        #[arg(long)]
        title: Option<String>,
        /// The new priority (optional, 1-4)
        #[arg(short, long, value_name = "PRIORITY")]
        priority: Option<u8>,
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
            Commands::Add { title, priority } => {
                if let Err(e) = TodoManager::validate_priority(priority) {
                    return Err(format!("❌ {e}"));
                }
                let todo = todo_manager.add_todo(title, priority)?;
                println!("✅ Added todo: {} (priority {})", todo.title, todo.priority);
                Ok(())
            }
            Commands::Edit {
                id,
                title,
                priority,
            } => {
                if let Some(p) = priority {
                    if let Err(e) = TodoManager::validate_priority(p) {
                        return Err(format!("❌ {e}"));
                    }
                }
                todo_manager.edit_todo(id, title, priority)?;
                println!("✏️  Todo {id} updated successfully");
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
                    let status = if todo.completed {
                        "✅ completed"
                    } else {
                        "⏳ incomplete"
                    };
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
            let colored_title = match todo.priority {
                1 => todo.title.red().bold(),
                2 => todo.title.yellow().bold(),
                3 => todo.title.blue().bold(),
                _ => todo.title.normal(),
            };
            println!("  {id} [{status}] {colored_title}");
        }
    }
}
