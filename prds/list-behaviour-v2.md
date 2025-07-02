# Todo CLI Application - List Behavior Enhancement PRD (v2)

## Project Overview
Enhance the Todo CLI application to provide a more intuitive user experience by making the list command the default behavior when no arguments are provided.

## Current Behavior
When running `tt` without any arguments, the application displays the usage/help text from clap, showing available commands and their descriptions.

## Proposed Change
When running `tt` without any arguments, the application should instead execute the list command, displaying all todo items (or the "no todos found" message if the list is empty).

## Rationale
- **User Experience**: Most users will want to see their todos when they run the application
- **Intuitive Design**: Following the principle of least surprise - users expect to see their data when opening an application
- **Efficiency**: Reduces the number of keystrokes needed for the most common operation
- **Consistency**: Aligns with common CLI patterns where the default action shows the current state

## Implementation Requirements

### Technical Changes
1. **CLI Structure Modification** (`src/cli.rs`)
   - Modify the `Cli` struct to make the command optional
   - Add a default implementation that triggers the list behavior
   - Ensure help text is still accessible via `tt --help` or `tt -h`

2. **Main Application Update** (`src/main.rs`)
   - Handle the case where no command is provided
   - Default to list behavior when no subcommand is specified

### User Interface
- **Command**: `tt` (no arguments)
- **Expected Output**: Same as `tt list`
  - If todos exist: Display numbered list with completion status
  - If no todos: Display "No todos found" message with helpful hint

### Backward Compatibility
- All existing commands must continue to work exactly as before
- Help functionality must remain accessible via explicit flags
- No breaking changes to existing command syntax

## Success Criteria
- [ ] Running `tt` without arguments lists todos instead of showing help
- [ ] Running `tt --help` or `tt -h` still shows the help text
- [ ] All existing commands (`add`, `list`, `complete`, etc.) work unchanged
- [ ] Empty todo list shows appropriate "no todos found" message
- [ ] User experience feels more intuitive and efficient

## Testing Requirements
- [ ] Test default behavior with empty todo list
- [ ] Test default behavior with existing todos
- [ ] Test that help is still accessible via flags
- [ ] Test that all existing commands work unchanged
- [ ] Test edge cases (very long todo titles, many todos, etc.)

## Implementation Notes
- This change affects only the CLI layer, not the core business logic
- The TodoManager and Todo model remain unchanged
- This is a user experience enhancement that doesn't require data persistence
- The change aligns with the principle of making the most common action the easiest to perform

## Future Considerations
- When persistence is implemented in Phase 2, this default behavior will become even more valuable
- Could be extended to show additional information in the default view (e.g., completion statistics)
- Could be enhanced to support filtering or sorting options in the default view

## Migration Strategy
- This is a non-breaking change that improves user experience
- No migration steps required for users
- Documentation should be updated to reflect the new default behavior 