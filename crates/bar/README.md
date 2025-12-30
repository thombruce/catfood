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

> [!NOTE]
> You may wish to try using catfood without the Kitty terminal emulator. Functionality is not guaranteed
> in this case, but you are free to experiment using the `--no-kitten` command line flag.

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

#### Component Structure

Each Lua component should return a table with the following structure:

```lua
return {
    -- Optional configuration
    config = {
        -- Component-specific settings
    },
    
    -- Optional update function (called periodically)
    update = function()
        -- Update internal state or fetch external data
    end,
    
    -- Required render function
    -- Returns text to display, optionally with color
    render = function(colorize)
        -- colorize: boolean indicating if colors should be used
        -- Returns either:
        -- 1. A string: "text"
        -- 2. A table: {"text", "color"}
        
        return {"12:34", "yellow"}
    end
}
```

#### Available Colors

Lua components can use the following color names:
- `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `black`
- `gray`/`grey`, `dark_red`, `dark_green`, `dark_yellow`, `dark_blue`, `dark_magenta`, `dark_cyan`, `dark_gray`

> ![NOTE]
> Color support is a work in progress. Not all of these colors may work. In future, user-defined colors may be supported along with fallback values for more restricted terminal color-spaces.

#### Example Components

##### Simple Clock Component

```lua
return {
    config = {
        show_seconds = true
    },
    
    render = function(colorize)
        local time = os.date("%H:%M")
        if true then -- show_seconds from config
            time = os.date("%H:%M:%S")
        end
        
        if colorize then
            local hour = tonumber(os.date("%H"))
            local color = "yellow"
            if hour < 6 or hour >= 18 then
                color = "magenta"
            end
            return {time, color}
        else
            return {time, nil}
        end
    end
}
```

##### System Uptime Component

```lua
return {
    update = function()
        -- Store uptime in component state (simplified)
        _uptime = io.open("/proc/uptime"):read("*a"):match("(%d+)")
    end,
    
    render = function(colorize)
        local uptime = tonumber(_uptime) or 0
        local hours = math.floor(uptime / 3600)
        local minutes = math.floor((uptime % 3600) / 60)
        local text = string.format("Uptime: %dh %dm", hours, minutes)
        
        if colorize then
            return {text, "green"}
        else
            return {text, nil}
        end
    end
}
```

#### Installation

1. Create the components directory:
   ```bash
   mkdir -p ~/.config/catfood/components
   ```

2. Add your Lua component files (`.lua` extension)
3. Reference them in `config.json` by filename (without extension)
4. Restart catfood_bar or wait for automatic config reload

#### Error Handling

- If a Lua component fails to load or has errors, it will display as `❌ component_name`
- Built-in components are unaffected by Lua component failures
- Check application logs for detailed error information

#### Limitations

- Lua components run in the same process as the main application
- Long-running operations in `update()` may affect UI responsiveness
- No direct file system or network access beyond standard Lua libraries (can be extended if needed)
- Component state is not persisted across restarts

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

## Advanced Configuration

### Component Reference

#### Special Components

**kitty_tabs** - Kitty terminal tabs with program icons
- Requires `kitty --single-instance` flag for proper detection
- Active tab: Shows full tab title (up to 20 chars)
- Inactive tabs: Shows program icons only for compact display
- Supports nvim, vim, htop, btop, git, ssh, cargo, and more
- Custom socket path configuration available:

```json
{
  "name": "kitty_tabs",
  "socket_path": "/tmp/custom-kitty-socket"
}
```

**separator** - Visual separator (" | ")
**space** - Single space character for fine spacing

### Sparkline Configuration

The `cpu`, `ram`, and `wifi` components support sparkline mode with these options:

- **`sparkline`** (default: false) - Enable sparkline mode
- **`sparkline_length`** (default: 10) - Graph width in characters  
- **`sparkline_update_freq`** (default: varies) - Update frequency in seconds
- **`sparkline_logarithmic`** (default: varies by component) - Use logarithmic scaling

#### Sparkline Scaling

**Linear Scaling** (default for CPU/RAM)
- Equal percentage changes have equal visual impact
- Best for bounded ranges like percentages

**Logarithmic Scaling** (default for WiFi)
- Equal ratio changes have equal visual impact  
- Best for data with high dynamic range (network traffic)
- Small values remain visible when large values are present

#### Scaling Examples

```json
// CPU with linear scaling (default)
{
  "name": "cpu",
  "sparkline": true,
  "sparkline_length": 10,
  "sparkline_update_freq": 3
}

// WiFi with logarithmic scaling (default)
{
  "name": "wifi",
  "sparkline": true,
  "sparkline_length": 15,
  "sparkline_update_freq": 2
}

// Override: CPU with logarithmic scaling
{
  "name": "cpu",
  "sparkline": true,
  "sparkline_length": 10,
  "sparkline_update_freq": 3,
  "sparkline_logarithmic": true
}

// Override: WiFi with linear scaling
{
  "name": "wifi",
  "sparkline": true,
  "sparkline_length": 15,
  "sparkline_update_freq": 2,
  "sparkline_logarithmic": false
}
```

### Additional Configuration Examples

#### Network Focus
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": ["wifi", "separator", "battery"]
  }
}
```

#### Terminal Workflow Focus
```json
{
  "bars": {
    "left": ["workspaces", "separator", "kitty_tabs"],
    "middle": ["time"],
    "right": ["wifi", "separator", "battery"]
  }
}
```

#### Full System Monitoring with Sparklines
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": [
      {
        "name": "cpu",
        "sparkline": true,
        "sparkline_length": 8,
        "sparkline_update_freq": 2
      },
      "space",
      {
        "name": "ram", 
        "sparkline": true,
        "sparkline_length": 8
      },
      "separator",
      {
        "name": "wifi",
        "sparkline": true,
        "sparkline_length": 10
      },
      "separator",
      "battery"
    ]
  }
}
```

#### Mixed Traditional and Sparkline Display
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time"],
    "right": [
      {
        "name": "cpu",
        "sparkline": true,
        "sparkline_length": 12
      },
      "separator",
      "ram",
      "separator",
      "wifi"
    ]
  }
}
```

#### Custom Grouping
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": [
      "temperature", "cpu", "ram",
      "separator", "separator",
      "wifi",
      "separator",
      "brightness", "volume", "battery"
    ]
  }
}
```

#### Fine-tuned Spacing
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "space", "separator", "space", "weather"],
    "right": [
      "temperature", "space", "cpu", "space", "ram",
      "separator",
      "wifi", "space",
      "separator",
      "brightness", "space", "volume", "space", "battery"
    ]
  }
}
```

#### Minimal with Custom Spacing
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "space", "weather"],
    "right": ["wifi", "space", "battery"]
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
