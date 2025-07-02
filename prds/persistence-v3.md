# Todo CLI Application - Persistence Layer PRD (v3)

## Project Overview
Add a persistence layer to the Todo CLI application that stores TodoItems in JSON format in the user's home directory, enabling todos to persist across application sessions while maintaining the existing CLI interface.

## Current State
- Todos are stored in-memory only and lost when the application exits
- TodoManager uses a simple `Vec<Todo>` for storage
- No persistence mechanism exists
- CLI interface is well-established and user-friendly

## Proposed Solution
Implement JSON-based persistence using `serde` to serialize/deserialize TodoItems to/from a `tt.json` file in the user's home directory.

## Requirements

### Core Requirements
1. **File Location**: Store todos in `~/.tt.json` (user's home directory)
2. **Format**: JSON serialization using `serde`
3. **Transparency**: No changes to existing CLI interface or user experience
4. **Reliability**: Fail fast on file I/O errors with clear error messages
5. **Backward Compatibility**: Seamless migration from in-memory to persistent storage

### Technical Requirements

#### 1. Todo Model Updates (`src/models/todo.rs`)
- Add `serde` derive macros to the `Todo` struct
- Ensure all fields are properly serializable
- Maintain existing public API and behavior

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: String, // ISO 8601 format
}
```

#### 2. TodoManager Updates (`src/todo_manager.rs`)
- Add persistence methods for loading and saving todos
- Implement automatic save on every modification
- Add error handling for file operations
- Maintain existing public API

**New Methods:**
- `load_from_file() -> Result<(), String>` - Load todos from JSON file
- `save_to_file() -> Result<(), String>` - Save todos to JSON file
- `ensure_file_exists() -> Result<(), String>` - Create file if it doesn't exist

**Modified Methods:**
- All CRUD operations should automatically save after modification
- Constructor should attempt to load existing todos

#### 3. Dependencies Update (`Cargo.toml`)
- Add `serde` and `serde_json` dependencies
- Ensure compatibility with existing `chrono` dependency

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### 4. File Structure
```
~/.tt.json
{
  "todos": [
    {
      "title": "Buy groceries",
      "completed": false,
      "created_at": "2024-01-15T10:30:00Z"
    },
    {
      "title": "Walk the dog",
      "completed": true,
      "created_at": "2024-01-15T09:00:00Z"
    }
  ]
}
```

## Implementation Strategy

### Phase 1: Core Persistence
1. **Add Dependencies**: Update `Cargo.toml` with serde dependencies
2. **Update Todo Model**: Add serde derive macros
3. **Enhance TodoManager**: Add persistence methods
4. **Update Main Application**: Initialize TodoManager with persistence

### Phase 2: Error Handling & Edge Cases
1. **File I/O Error Handling**: Fail fast with clear error messages for permission issues, disk full, etc.
2. **Corrupted File Handling**: Show error to user with option to start fresh
3. **Migration Strategy**: Handle first-time users vs. existing users

### Phase 3: Testing & Validation
1. **Unit Tests**: Test all persistence methods
2. **Integration Tests**: Test full save/load cycle
3. **Error Scenario Tests**: Test various failure modes

## User Experience

### No Changes Required
- All existing CLI commands work exactly as before
- Same output format and behavior
- Same error messages and feedback
- Same help text and documentation

### New Behaviors
- Todos persist across application restarts
- First run creates the `~/.tt.json` file automatically
- File permissions and location are handled transparently

### Error Scenarios
- **File Permission Issues**: Clear error message with guidance, application exits
- **Corrupted JSON**: Show error to user with option to delete file and start fresh
- **Disk Full**: Clear error message, application exits
- **Home Directory Unavailable**: Clear error message, application exits

## Success Criteria
- [ ] Todos persist across application restarts
- [ ] All existing CLI commands work unchanged
- [ ] File is created automatically on first use
- [ ] Fail fast with clear error messages for file I/O issues
- [ ] JSON format is human-readable and well-structured
- [ ] No performance degradation in normal usage
- [ ] Comprehensive test coverage for persistence layer

## Testing Requirements

### Unit Tests
- [ ] Todo serialization/deserialization
- [ ] TodoManager persistence methods
- [ ] Error handling for various failure scenarios
- [ ] File creation and loading logic

### Integration Tests
- [ ] Full save/load cycle with multiple todos
- [ ] Application restart with existing todos
- [ ] Error recovery scenarios
- [ ] Performance with large todo lists

### Manual Testing
- [ ] First-time user experience
- [ ] Existing user migration
- [ ] File permission scenarios
- [ ] Cross-platform compatibility (macOS, Linux, Windows)

## Technical Considerations

### File Location
- **Unix-like systems**: `~/.tt.json`
- **Windows**: `%USERPROFILE%\.tt.json`
- Use `dirs` crate for cross-platform home directory detection
- **Fallback**: If home directory unavailable, fail with clear error message

### JSON Structure
- Wrap todos in an object for future extensibility
- Use consistent field naming
- Maintain ISO 8601 timestamp format

### Performance
- Save operations should be fast enough for interactive use
- Immediate saves on every modification (keep it simple)
- Minimize file I/O overhead

### Security
- Set appropriate file permissions (600 for user-only access on Unix-like systems)
- Validate JSON structure before deserialization
- Handle potential security issues with file operations

### Error Handling Strategy
- **Fail Fast**: Application exits with clear error message on file I/O failures
- **Corrupted Files**: Show error to user with option to delete file and start fresh
- **No Silent Failures**: All errors should be visible to the user

## Migration Strategy
- **New Users**: Automatic file creation on first todo operation
- **Existing Users**: Seamless transition - todos will persist after first save
- **No Breaking Changes**: All existing functionality preserved

## Future Considerations
- **Backup Strategy**: Consider automatic backup creation
- **File Format Versioning**: Prepare for future format changes
- **Cloud Sync**: Structure supports future cloud integration
- **Multiple Lists**: JSON structure allows for multiple todo lists
- **Export/Import**: Easy to add export to other formats

## Dependencies to Add
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"  # For cross-platform home directory detection
```

## Risk Assessment
- **Low Risk**: Well-established serde ecosystem
- **Low Risk**: No changes to public API
- **Medium Risk**: File I/O error handling complexity
- **Low Risk**: Backward compatibility maintained

## Timeline
- **Phase 1**: 1-2 days for core implementation
- **Phase 2**: 1 day for error handling
- **Phase 3**: 1 day for testing and validation
- **Total**: 3-4 days for complete feature 