# Development Commands

## IMPORTANT: Repository Policy
- NEVER commit or push code without explicit user permission
- Always run `cargo clippy` and `cargo fmt` when making changes

## Build, Test, and Lint
- `cargo build` - Build the project
- `cargo run` - Run the application  
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo fmt -- --check` - Check code formatting
- `cargo fmt` - Format code
- `cargo clippy` - Run lints

## Code Style Guidelines

### Imports
- Group imports: std library, external crates, local modules
- Use `use` statements at file top, organize alphabetically
- Prefer specific imports over `use std::prelude::*`

### Naming
- `PascalCase` for structs and enums
- `snake_case` for functions and variables  
- `SCREAMING_SNAKE_CASE` for constants
- Use descriptive names, avoid abbreviations

### Error Handling
- Use `color_eyre::Result<()>` for main functions
- Use `?` operator for error propagation
- Handle errors gracefully with appropriate logging

### Component Structure
- Components follow pattern: `new()`, `update()`, `render()`
- Use `#[derive(Debug)]` for structs
- Keep components focused on single responsibility
- Use ratatui widgets with `.fg(Color::White)` styling

### Documentation
- Add doc comments for public APIs (`///`)
- Keep comments concise and focused on "why", not "what"