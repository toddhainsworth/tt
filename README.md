# Todo CLI Application (tt)

[![CI](https://github.com/toddhainsworth/tt/workflows/CI/badge.svg)](https://github.com/toddhainsworth/tt/actions?query=workflow%3ACI)

A simple and efficient command-line Todo application built in Rust that helps you manage your tasks directly from the terminal.

## Features

- ✅ **Add todos** with descriptive titles and priorities (1-4)
- 📝 **List all todos** with completion status and color-coded priority
- 🏃 **List todos by default when running `tt` with no arguments** (as of v2)
- 🔄 **Toggle completion status** with a single command
- ✅ **Mark todos as complete/incomplete** explicitly
- 🗑️ **Delete todos** by ID
- 📅 **Automatic timestamps** for when todos are created
- 💾 **Automatic persistence**: Todos are saved to a JSON file in your home directory (`~/.tt.json`) and persist across sessions
- ⭐ **Priorities**: Mark important todos (1-4, color-coded)
- 🎨 **Beautiful CLI interface** with emojis, color, and clear feedback

## Priority Levels & Color Coding

Each todo can have a priority from 1 (highest) to 4 (lowest):

| Priority | Meaning         | Color   |
|----------|----------------|---------|
| 1        | Highest        | Red     |
| 2        | High           | Yellow  |
| 3        | Medium         | Blue    |
| 4        | Lowest (default)| Default |

Todos are displayed in the list command with their title color-coded by priority.

## Installation

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Build from Source
```bash
# Clone the repository
git clone <repository-url>
cd tt

# Build the application
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Commands

```bash
# List all todos (default)
tt

# Add a new todo (default priority 4)
tt add "Buy groceries"

# Add a new todo with priority 1 (highest)
tt add "Pay bills" --priority 1

# Edit a todo's title and/or priority
tt edit 0 --title "Pay rent" --priority 2

# List all todos (explicit)
tt list

# Mark a todo as completed (by ID)
tt complete 0

# Mark a todo as incomplete (by ID)
tt incomplete 0

# Toggle a todo's completion status (by ID)
tt toggle 0

# Delete a todo (by ID)
tt delete 0

# Show help
tt --help
```

### Example Workflow

```bash
# Start with an empty list
tt
# Output: 📝 No todos found. Add one with `tt add <title>`

# Add some todos
tt add "Buy milk" --priority 2
tt add "Walk the dog" --priority 1
tt add "Read Rust book"

# List todos (default)
tt
# Output:
# 📝 Your todos:
#   0 [⏳] Buy milk        # yellow (priority 2)
#   1 [⏳] Walk the dog   # red (priority 1)
#   2 [⏳] Read Rust book # default (priority 4)

# Edit a todo's priority
tt edit 2 --priority 3

# Complete a task
tt complete 1
# Output: ✅ Marked as completed: Walk the dog

# List todos
tt
# Output:
# 📝 Your todos:
#   0 [⏳] Buy milk        # yellow (priority 2)
#   1 [✅] Walk the dog   # red (priority 1)
#   2 [⏳] Read Rust book # blue (priority 3)

# Delete a task
tt delete 0
# Output: 🗑️ Todo deleted successfully

# Final list
tt
# Output:
# 📝 Your todos:
#   0 [✅] Walk the dog   # red (priority 1)
#   1 [⏳] Read Rust book # blue (priority 3)
```

### Data Persistence

- **Automatic**: All your todos are saved automatically to a file in your home directory (`~/.tt.json` on Unix/macOS, or the equivalent on Windows)
- **No manual action required**: Todos persist across application restarts
- **Human-readable**: The file is in JSON format and can be inspected or backed up manually if desired

## Project Structure

```
src/
├── main.rs              # Application entry point
├── cli.rs               # CLI command handling
├── todo_manager.rs      # Todo business logic and persistence
└── models/
    ├── mod.rs           # Module declarations
    └── todo.rs          # Todo data structure and serialization
```

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_add_todo
```

### Testing Strategy
Tests are written using Rust's built-in testing framework and follow the convention of placing test modules in the same file as the code they test:

- **Unit tests** are located in `#[cfg(test)]` modules within each source file
- **Todo model tests** cover creation, completion status changes, and Default implementation
- **TodoManager tests** cover all CRUD operations, error handling, and persistence
- Tests ensure proper error handling for invalid IDs and edge cases

### Running in Development Mode
```bash
cargo run -- <command>
```

## Dependencies

- **clap**: Command-line argument parsing
- **chrono**: Date and time handling
- **serde**: Serialization/deserialization for persistence
- **serde_json**: JSON file handling
- **dirs**: Cross-platform home directory detection
- **colored**: Terminal color output for priorities

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Roadmap

### Phase 2 Features (Planned)
- 🔍 **Search**: Find todos by title
- 🏷️ **Categories**: Organize todos with tags
- 📅 **Due dates**: Set deadlines for todos
- 🔄 **Sorting**: Sort by date, priority, or status

---

**Built with ❤️ in Rust** 

**Built with ❤️ with Cursor** 