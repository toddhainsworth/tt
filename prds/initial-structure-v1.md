# Todo CLI Application - Initial Structure PRD

## Project Overview
A command-line interface (CLI) Todo application built in Rust that allows users to manage their tasks efficiently through terminal commands.

## Core Requirements

### Todo Item Structure
Each Todo item must contain:
- **Title**: A string representing the task description
- **Completed**: A boolean value indicating completion status (false by default)
- **Created At**: A date string storing the creation timestamp

### Initial Project Structure

#### 1. Core Data Model (`src/models/todo.rs`)
```rust
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: String, // ISO 8601 format
}

impl Todo {
    pub fn new(title: String) -> Self { /* ... */ }
    pub fn toggle_completed(&mut self) { /* ... */ }
    pub fn set_completed(&mut self, value: bool) { /* ... */ }
}

impl Default for Todo {
    fn default() -> Self { /* ... */ }
}
```

#### 2. Todo Manager (`src/todo_manager.rs`)
- CRUD operations for Todo items
- In-memory storage for initial implementation
- Methods:
  - `add_todo(title: String) -> Todo`
  - `list_todos() -> Vec<Todo>`
  - `mark_completed(id: usize) -> Result<(), String>`
  - `mark_incomplete(id: usize) -> Result<(), String>`
  - `toggle_completed(id: usize) -> Result<(), String>`
  - `delete_todo(id: usize) -> Result<(), String>`

#### 3. CLI Interface (`src/cli.rs`)
- Command-line argument parsing using `clap` crate
- Commands:
  - `add <title>` - Add a new todo
  - `list` - List all todos
  - `complete <id>` - Mark todo as completed
  - `incomplete <id>` - Mark todo as incomplete
  - `toggle <id>` - Toggle todo completion status
  - `delete <id>` - Delete a todo
  - `help` - Show help information

#### 4. Main Application (`src/main.rs`)
- Entry point that orchestrates CLI parsing and Todo operations
- Error handling and user feedback

## Dependencies
- `clap` - For CLI argument parsing
- `chrono` - For date/time handling
- `serde` + `serde_json` - For future JSON persistence (optional for initial version)

## File Structure
```
src/
├── main.rs              # Application entry point
├── cli.rs               # CLI command handling
├── todo_manager.rs      # Todo business logic
└── models/
    ├── mod.rs           # Module declarations
    └── todo.rs          # Todo data structure
```

## Implementation Phases

### Phase 1: Core Structure (Current) ✅
1. ✅ Define Todo struct with required fields
2. ✅ Create basic TodoManager with in-memory storage
3. ✅ Implement basic CLI structure
4. ✅ Add minimal CRUD operations
5. ✅ Add toggle functionality for completion status
6. ✅ Implement Default trait for Todo struct

### Phase 2: Enhanced Features (Future)
1. Add persistence (JSON file storage)
2. Implement filtering and sorting
3. Add due dates and priorities
4. Add categories/tags
5. Implement search functionality
6. Integrate `anyhow` crate for improved error handling

## Success Criteria
- [x] Todo items can be created with title, completion status, and creation timestamp
- [x] CLI commands work correctly for all CRUD operations
- [x] Application provides clear error messages and user feedback
- [x] Code follows Rust best practices and conventions
- [x] Application is well-documented and maintainable
- [x] Todo completion status can be toggled and explicitly set
- [x] Default trait implementation for Todo struct

## Technical Considerations
- Use proper error handling with `Result<T, E>` types
- Implement meaningful user feedback for all operations
- Follow Rust naming conventions and module organization
- Ensure the application is easy to extend for future features
- Provide flexible completion status management (toggle, set, mark complete/incomplete) 