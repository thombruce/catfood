# Catfood Bar =^,^=

![Catfood Bar](../../assets/catfood-bar.png)

A customizable system status bar built with Rust and extensible with Lua. Part of the catfood utility suite.

## Features

- **Modular Components**: Choose from 13+ built-in components or create custom Lua components
- **Live Configuration**: Hot-reload config changes without restarting
- **Sparkline Visualization**: Real-time usage graphs for system metrics
- **Multi-Bar Layout**: Left, middle, and right bar sections
- **Color Support**: Optional colorize mode (TODO: Configurable color themes)
- **Error Handling**: Graceful error display and logging

## Installation

### Dependencies

- Rust 1.70+
- Hyprland
- NetworkManager
- lm-sensors (for component temperatures)
- kitty terminal (makes use of `kitten panel` to render the bar)

> ![NOTE]
> Whereas the `catfood bar` utility does directly depend on Kitty, the `catfood-bar` binary may work without it.
> You will need to handle your own layering and positioning if you give this a try.

### From crates.io

```sh
# Install the bar component
cargo install catfood-bar
```

### From Source

```sh
# Build and install
cargo install --path crates/bar

# Or build and run locally
cargo run --release --bin catfood-bar
```

## Usage

Run the system bar:

```sh
catfood-bar
```

When run through the main `catfood` CLI, it automatically spawns in a kitty panel:

```sh
# Top of screen (default)
catfood bar
```

For other purposes, you can still run the `catfood-bar` binary independently:

```sh
# Bottom of screen
kitten panel --edge=bottom catfood-bar
```

## Configuration

Configuration is stored in `~/.config/catfood/bar.json`. The first run creates a default config with all components enabled.

### Basic Configuration

```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": [
      "temperature",
      "cpu",
      "ram",
      "separator",
      "wifi",
      "separator",
      "brightness",
      "volume",
      "separator",
      "battery"
    ]
  },
  "colorize": true
}
```

### Component Types

Components can be specified as either strings or objects with additional options:

#### String Components
```json
{
  "bars": {
    "right": ["cpu", "ram", "wifi"]
  }
}
```

#### Object Components with Options
```json
{
  "bars": {
    "right": [
      {
        "name": "cpu",
        "sparkline": true,
        "sparkline_length": 8,
        "sparkline_update_freq": 2
      }
    ]
  }
}
```

## Available Components

### System Monitoring
- **`workspaces`** - Hyprland workspace switching
- **`windows`** - Window management info
- **`cpu`** - CPU usage percentage (supports sparkline)
- **`ram`** - Memory usage percentage (supports sparkline)
- **`temperature`** - CPU temperature
- **`battery`** - Battery status and percentage

### Network
- **`wifi`** - WiFi connection status and signal strength (supports sparkline)
- **`weather`** - Current weather information

### Audio & Display
- **`volume`** - System volume level
- **`brightness`** - Screen brightness

### Time & Date
- **`time`** - Current date and time

### Layout Components
- **`separator`** - Visual separator (" | ")
- **`space`** - Single space character for fine spacing

### Sparkline Components

The following components support sparkline visualization:

- **`cpu`** - Real-time CPU usage graph
- **`ram`** - Memory usage over time
- **`wifi`** - WiFi signal strength history

#### Sparkline Options

- **`sparkline`** (default: false) - Enable sparkline mode
- **`sparkline_length`** (default: 10) - Graph width in characters
- **`sparkline_update_freq`** (default: varies) - Update frequency in seconds
  - CPU: 3 seconds
  - RAM: 2 seconds
  - WiFi: 2 seconds
- **`sparkline_logarithmic`** (default: false) - Use logarithmic scaling

### Custom Lua Components

Create custom components in `~/.config/catfood/components/*.lua`:

```lua
return {
    config = {
        -- Component-specific settings
    },
    
    update = function()
        -- Update state periodically (optional)
    end,
    
    render = function(colorize)
        -- Return {"text", "color"} or "text"
        return {"12:34", "yellow"}
    end
}
```

Available colors: `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `black`, `gray`, `dark_red`, `dark_green`, `dark_yellow`, `dark_blue`, `dark_magenta`, `dark_cyan`, `dark_gray`.

> ![NOTE]
> Color support is a work in progress. Not all of these colors may work. In future, user-defined colors may be supported along with fallback values for more restricted terminal color-spaces.

## Configuration Examples

### Minimal Setup
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time"],
    "right": ["battery"]
  }
}
```

### System Monitoring Focus
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "temperature"],
    "right": [
      {"name": "cpu", "sparkline": true, "sparkline_length": 8},
      "space",
      {"name": "ram", "sparkline": true, "sparkline_length": 8},
      "separator",
      {"name": "wifi", "sparkline": true, "sparkline_length": 10}
    ]
  }
}
```

### Compact Layout
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time"],
    "right": [
      {"name": "cpu", "sparkline": true, "sparkline_length": 4},
      "space",
      {"name": "ram", "sparkline": true, "sparkline_length": 4},
      "space",
      {"name": "wifi", "sparkline": true, "sparkline_length": 4}
    ]
  }
}
```

## Hot-Reload

Configuration changes are applied automatically:
1. Edit `~/.config/catfood/bar.json`
2. Save the file
3. Changes appear instantly without restarting

## Logging

Errors are logged to `~/.local/share/catfood/logs/bar.log`:
```
2025-12-21T03:45:12Z [ERROR] [COMPONENT_WORKSPACES] Error: Failed to get workspaces
2025-12-21T03:45:13Z [ERROR] [CONFIG] Failed to reload configuration: ...
```

The log maintains the last 1000 lines.

## License

Copyright (c) Thom Bruce <thom@thombruce.com>

Licensed under the MIT license.
