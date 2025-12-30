# Catfood =^,^=

![Catfood Bar](assets/catfood-bar.png)

A modular utility suite for system management, built with Rust and Ratatui.

## Usage

### System Bar

Run the system bar:

Top of screen:
```sh
kitten panel catfood bar
```

Bottom of screen:
```sh
kitten panel --edge=bottom catfood bar
```

### Other Commands

```sh
catfood bar                    # Run the system bar (spawns in kitten panel)
catfood bar --no-kitten         # Run the system bar directly in current terminal
catfood menu                   # Run menu system (coming soon)
catfood notifications             # Run notification system (coming soon)
```

### Direct Binary Usage

Both `catfood bar` and `catfood-bar` support the same flags:

```sh
catfood-bar                    # Run in kitten panel (default)
catfood-bar --no-kitten         # Run directly in current terminal
```

## Installation

### From crates.io

```sh
# Install the complete catfood suite
cargo install catfood-bar
cargo install catfood

# Install just the bar component (standalone)
cargo install catfood-bar
```

**Note**: `catfood bar` requires `catfood-bar` to be installed separately. This allows standalone installation of individual components while maintaining the full suite functionality.

### From Source

```sh
# Build and install all utilities
cargo install --path .

# Or build and run locally
cargo run --release -- bar
```

## Configuration

catfood supports live configuration via `~/.config/catfood/bar.json`. The first time you run the application, a default configuration file will be created.

### Hot-Reload

Configuration changes are automatically detected and applied without restarting the application. Simply edit your `bar.json` file and the bar layout will update in real-time!

### Error Logging

All errors are logged to `~/.local/share/catfood/logs/bar.log` in ISO format:
```
2025-12-21T03:45:12Z [ERROR] [COMPONENT_WORKSPACES] Error: Failed to get workspaces
2025-12-21T03:45:13Z [ERROR] [CONFIG] Failed to reload configuration: ...
2025-12-21T03:45:14Z [ERROR] [FILE_WATCHER] Failed to create file watcher: ...
```

The log file keeps only the last 1000 lines.

See the [catfood-bar README](crates/bar/README.md) for detailed configuration options and examples.

## Release Process

This project uses automated releases triggered by git tags:

### Making a Release

1. **Update Version** (using conventional commits):
   ```bash
   # Bump version and update workspace
   cargo release patch  # or minor/major
   ```

2. **Create Tag**:
   ```bash
   # Tag the release (must be on main branch)
   git tag v0.1.1
   git push --tags
   ```

3. **Automatic Release**:
   - GitHub Actions builds and tests on all platforms
   - Binaries are compiled for Linux, macOS, and Windows
   - Workspace is published to crates.io concurrently
   - GitHub Release is created with binaries and changelog

### Development Workflow

Use conventional commit messages for automatic changelog generation:

- `feat: add new weather component` → New features
- `fix: memory leak in CPU monitoring` → Bug fixes  
- `docs: update configuration examples` → Documentation
- `refactor: improve component performance` → Code improvements

### Release Types

- **Patch (0.1.1)**: Bug fixes, documentation updates
- **Minor (0.2.0)**: New features, significant enhancements
- **Major (1.0.0)**: Breaking changes, API changes

## License

Copyright (c) Thom Bruce <thom@thombruce.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
