# Todo CLI Application - Priorities Feature PRD (v5)

## Project Overview
Add priority levels to TodoItems in the Todo CLI application, allowing users to assign priorities from 1-4 (where 1 is highest priority) and visually distinguish them through color-coded output in the list command.

## Current State
- Todos have basic properties: title, completed status, and creation timestamp
- List command displays todos with simple status indicators (✅/⏳)
- No priority or importance ranking system exists
- All todos are treated equally in terms of visual presentation

## Proposed Solution
Implement a priority system that allows users to assign priority levels (1-4) to todos, with visual differentiation through color-coded output in the list command. Priority 1 is highest priority, priority 4 is lowest priority.

## Requirements

### Core Requirements
1. **Priority Assignment**: Allow setting priority (1-4) when adding or editing todos
2. **Visual Distinction**: Color-code todos in list output based on priority
3. **Backward Compatibility**: Existing todos default to priority 4 (lowest)
4. **CLI Integration**: Add priority options to add and edit commands
5. **Persistence**: Priority values are saved and restored with todos

### Technical Requirements

#### 1. Todo Model Updates (`src/models/todo.rs`)
- Add `priority` field to the `Todo` struct
- Ensure proper serialization/deserialization
- Maintain backward compatibility with existing todos

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: String, // ISO 8601 format
    pub priority: u8,       // 1-4, where 1 is highest priority
}
```

#### 2. CLI Interface Updates (`src/cli.rs`)
- Add priority parameter to `Add` command
- Add priority parameter to new `Edit` command
- Update help text and documentation

**New Command Structure:**
```rust
Commands::Add {
    title: String,
    #[arg(short, long, value_name = "PRIORITY", default_value = "4")]
    priority: Option<u8>,
},
Commands::Edit {
    id: usize,
    title: Option<String>,
    #[arg(short, long, value_name = "PRIORITY")]
    priority: Option<u8>,
},
```

#### 3. Display Logic Updates (`src/cli.rs`)
- Implement color-coded output for different priority levels
- Use terminal colors to distinguish priorities
- Maintain readability and accessibility

**Priority Color Scheme:**
- Priority 1 (Highest): Red text
- Priority 2 (High): Yellow text  
- Priority 3 (Medium): Blue text
- Priority 4 (Lowest): Default text color

#### 4. TodoManager Updates (`src/todo_manager.rs`)
- Add priority validation logic
- Update todo creation and modification methods
- Ensure proper error handling for invalid priorities

**New Methods:**
- `validate_priority(priority: u8) -> Result<(), String>` - Validate priority range
- `edit_todo(id: usize, title: Option<String>, priority: Option<u8>) -> Result<(), String>` - Edit existing todo

#### 5. Dependencies Update (`Cargo.toml`)
- Add `colored` crate for terminal color support
- Ensure cross-platform compatibility

```toml
[dependencies]
colored = "2.0"
```

## Implementation Strategy

### Phase 1: Core Priority System
1. **Update Todo Model**: Add priority field with default value
2. **Add Priority Validation**: Implement range checking (1-4)
3. **Update TodoManager**: Add priority support to CRUD operations
4. **Add Edit Command**: Implement todo editing functionality

### Phase 2: CLI Integration
1. **Update Add Command**: Add optional priority parameter
2. **Add Edit Command**: Allow modifying title and priority
3. **Update Help Text**: Document new priority options
4. **Add Validation**: Ensure priority values are within valid range

### Phase 3: Visual Enhancement
1. **Implement Color Coding**: Add colored output for different priorities
2. **Update Display Logic**: Modify list command to show priority colors
3. **Test Cross-Platform**: Ensure colors work on different terminals
4. **Add Fallback**: Graceful degradation for terminals without color support

### Phase 4: Testing & Polish
1. **Unit Tests**: Test priority validation and todo operations
2. **Integration Tests**: Test full priority workflow
3. **Manual Testing**: Verify color output on different platforms
4. **Documentation**: Update README with priority feature

## User Experience

### New Commands
```bash
# Add todo with priority
tt add "Important meeting" --priority 1
tt add "Buy groceries" -p 2

# Edit existing todo
tt edit 0 --priority 1
tt edit 0 -p 3 --title "Updated title"

# List todos (now with color coding)
tt list
```

### Visual Output
```
📝 Your todos:
   0 [⏳] Important meeting          # Red text (Priority 1)
   1 [✅] Buy groceries             # Yellow text (Priority 2)  
   2 [⏳] Read documentation        # Blue text (Priority 3)
   3 [⏳] Organize desk             # Default color (Priority 4)
```

### Backward Compatibility
- Existing todos automatically get priority 4 (lowest)
- All existing commands work unchanged
- No breaking changes to current functionality

### Error Handling
- Invalid priority values (0, 5+) show clear error messages
- Non-numeric priority values handled gracefully
- Missing priority defaults to 4 (lowest)

## Success Criteria
- [ ] Users can assign priorities 1-4 to todos
- [ ] Priority values are visually distinguished with colors
- [ ] Existing todos work without modification (default to priority 4)
- [ ] Priority values persist across application restarts
- [ ] Invalid priority values are rejected with clear error messages
- [ ] Colors work on major platforms (macOS, Linux, Windows)
- [ ] Graceful fallback for terminals without color support
- [ ] Comprehensive test coverage for priority functionality

## Testing Requirements

### Unit Tests
- [ ] Priority validation (1-4 range)
- [ ] Todo creation with different priorities
- [ ] Todo editing with priority changes
- [ ] Default priority assignment for existing todos
- [ ] Serialization/deserialization with priority field

### Integration Tests
- [ ] Full priority workflow (add, edit, list, persist)
- [ ] Backward compatibility with existing todo files
- [ ] Error handling for invalid priority values
- [ ] Color output verification

### Manual Testing
- [ ] Color display on different terminal types
- [ ] Cross-platform compatibility
- [ ] User experience with priority assignment
- [ ] Error message clarity and helpfulness

## Technical Considerations

### Priority Range
- **Valid Range**: 1-4 (inclusive)
- **Default Value**: 4 (lowest priority)
- **Validation**: Reject values outside valid range with clear error message
- **Type**: `u8` for efficiency and range appropriateness

### Color Implementation
- **Library**: Use `colored` crate for cross-platform color support
- **Colors**: Red (1), Yellow (2), Blue (3), Default (4)
- **Fallback**: Detect color support and fall back to text indicators if needed
- **Accessibility**: Ensure sufficient contrast for readability

### Backward Compatibility
- **Existing Todos**: Automatically assigned priority 4
- **File Format**: JSON structure remains compatible
- **CLI Commands**: All existing commands work unchanged
- **Migration**: Seamless transition for existing users

### Performance
- **Minimal Overhead**: Priority field adds negligible performance impact
- **Color Rendering**: Efficient color application without affecting core functionality
- **Validation**: Fast priority range checking

### Error Handling Strategy
- **Invalid Priorities**: Clear error message with valid range explanation
- **Non-numeric Input**: Graceful handling with helpful error message
- **Color Failures**: Fall back to text-based priority indicators
- **File Corruption**: Handle corrupted priority values gracefully

## Migration Strategy
- **New Users**: Full priority functionality available immediately
- **Existing Users**: Todos automatically get priority 4, can be edited later
- **File Format**: JSON structure remains backward compatible
- **No Breaking Changes**: All existing functionality preserved

## Future Considerations
- **Priority Sorting**: Sort todos by priority in list output
- **Priority Filtering**: Show only todos of specific priority levels
- **Priority Statistics**: Show priority distribution in list output
- **Custom Priority Names**: Allow users to name priority levels (e.g., "Critical", "High", "Medium", "Low")
- **Priority-based Reminders**: Future integration with due dates and reminders

## Dependencies to Add
```toml
[dependencies]
colored = "2.0"
```

## Risk Assessment

### Low Risk
- **Backward Compatibility**: Existing functionality preserved
- **Core Logic**: Priority system is straightforward to implement
- **Testing**: Comprehensive test coverage mitigates regressions

### Medium Risk
- **Color Support**: Different terminals may handle colors differently
- **Cross-Platform**: Color rendering may vary across platforms
- **User Experience**: Priority system adds complexity to CLI interface

### Mitigation Strategies
- **Thorough Testing**: Test on multiple platforms and terminal types
- **Graceful Degradation**: Fall back to text indicators if colors fail
- **Clear Documentation**: Provide examples and help text for new features
- **Incremental Rollout**: Implement in phases to catch issues early

## Implementation Timeline
- **Phase 1 (Core)**: 2-3 days - Basic priority system and model updates
- **Phase 2 (CLI)**: 2-3 days - Command-line interface integration
- **Phase 3 (Visual)**: 2-3 days - Color coding and display updates
- **Phase 4 (Testing)**: 2-3 days - Comprehensive testing and documentation
- **Total**: 8-12 days for complete implementation 