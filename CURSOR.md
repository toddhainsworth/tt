# Cursor Guidelines

This document provides guidelines for Cursor instances working with this repository and user.

## Communication & Workflow Preferences

### Always Confirm Before Committing
- **Never commit and push automatically** without explicit confirmation from the user
- Ask for confirmation before committing changes: "Would you like me to commit and push this change?"
- Be patient with iterative changes - the user may want to refine details before finalizing

### Code Quality Standards
- Ensure all code passes:
  - `cargo fmt --check --all` (formatting)
  - `cargo clippy -- -D warnings` (linting)
  - `cargo test` (all tests passing)
- Fix CI issues promptly when builds fail
- Confirm fixes work before pushing

## Project-Specific Patterns

### PRD Documentation
- Follow the established PRD format in the `prds/` directory
- Use the same structure and style as existing PRDs
- Include comprehensive requirements, implementation strategy, and testing plans

### Code Architecture
- Maintain backward compatibility with existing functionality
- Prefer simple, explicit APIs over complex helper methods
- Use conventional commit messages (feat:, fix:, style:, docs:, etc.)

### Documentation Updates
When adding features, comprehensively update:
- Features list in README
- Usage examples with realistic scenarios
- Dependencies list
- Roadmap status (move implemented features from planned to implemented)

## Rust-Specific Guidelines

### API Design
- Prefer explicit constructors like `Todo::new(title, priority)` over multiple helper methods
- Use idiomatic Rust patterns (e.g., `!(1..=4).contains(&priority)` instead of manual range checks)
- Follow clippy suggestions for modern Rust practices

### Error Handling
- Provide clear, user-friendly error messages
- Use proper Result types and error propagation
- Include helpful error context

### Testing
- Maintain comprehensive test coverage
- Test both success and error cases
- Update tests when changing APIs

## Attribution & Credits
- Add appropriate credits when requested (e.g., "Built with ❤️ with Cursor")
- Follow the user's preferred format for acknowledgments

## Key Commands to Remember
```bash
# Quality checks
cargo fmt --check --all
cargo clippy -- -D warnings
cargo test

# Apply formatting
cargo fmt --all

# Development workflow
cargo run -- <command>
cargo build --release
```

## Communication Style
- Be direct and efficient
- Ask for confirmation before major actions
- Provide clear explanations for technical decisions
- Respect the user's iterative refinement process

---

**Remember**: The user values clean, well-tested code and prefers to review changes before they're committed. Always ask before committing and pushing! 