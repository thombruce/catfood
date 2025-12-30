# CatfoodBar Configuration Example

This configuration file allows you to customize which components appear in each bar section.

## Location
`~/.config/catfood/bar.json`

## Available Components
- `workspaces` - Hyprland workspaces
- `windows` - Application windows with icons
- `kitty_tabs` - Kitty terminal tabs with program icons (requires `--single-instance` flag)
  - Active tab: Shows full tab title (up to 20 chars)
  - Inactive tabs: Shows program icons only for compact display
  - Supports nvim, vim, htop, btop, git, ssh, cargo, and more
- `time` - Current date and time
- `weather` - Weather information
- `temperature` - CPU temperature
- `cpu` - CPU usage (supports sparkline mode)
- `ram` - Memory usage (supports sparkline mode)
- `wifi` - WiFi connection status (supports sparkline mode)
- `brightness` - Screen brightness
- `volume` - Volume level
- `battery` - Battery status
- `separator` - Visual separator (" | ") for creating custom sections
- `space` - Single space character (" ") for fine-tuned spacing

**Sparkline Support**: The `cpu`, `ram`, and `wifi` components support sparkline mode to visualize usage patterns over time. See the configuration examples below for details.

**KittyTabs Component Requirements**: The `kitty_tabs` component requires Kitty terminal instances to be started with the `--single-instance` flag for proper detection. This allows the component to identify and communicate with individual Kitty processes via their UNIX sockets.

```sh
# Launch Kitty with single instance for kitty_tabs support
kitty --single-instance
```

## Configuration Structure

The configuration supports both simple string components and object-based components with additional options.

### Basic Structure
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
  }
}
```

### Component Objects
Components can also be configured as objects with additional options:

```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": [
      "temperature",
      "separator", 
      {
        "name": "wifi",
        "sparkline": true,
        "sparkline_length": 8,
        "sparkline_update_freq": 1
      },
      "separator",
      "battery"
    ]
  }
}
```

#### Sparkline Component Options
The following components support sparkline mode: **cpu**, **ram**, **wifi**

For any sparkline-enabled component:
- **`name`** (required): Component name ("cpu", "ram", or "wifi")
- **`sparkline`** (optional, default: false): Enable sparkline mode to show usage over time
- **`sparkline_length`** (optional, default: 10): Length of the sparkline in characters
- **`sparkline_update_freq`** (optional, default: varies by component): Update frequency in seconds
  - CPU: 3 seconds default
  - RAM: 2 seconds default  
  - WiFi: 2 seconds default

## Customization Examples

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
    "middle": ["time", "separator", "temperature", "separator", "cpu", "separator", "ram"],
    "right": ["wifi", "separator", "battery"]
  }
}
```

### Network Focus
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": ["wifi", "separator", "separator", "battery"]
  }
}
```

### Terminal Workflow Focus
```json
{
  "bars": {
    "left": ["workspaces", "separator", "kitty_tabs"],
    "middle": ["time"],
    "right": ["wifi", "separator", "battery"]
  }
}
```

### Compact Terminal Display
The `kitty_tabs` component uses a compact display approach:
- **Active tab**: Shows full title with highlight (20 chars max)
- **Inactive tabs**: Show only program icons (12 chars max for titles when not showing icons)

This keeps the bar concise while still showing what's important at a glance.

### WiFi with Sparkline
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "separator", "weather"],
    "right": [
      "separator",
      {
        "name": "wifi",
        "sparkline": true,
        "sparkline_length": 12,
        "sparkline_update_freq": 1
      },
      "separator",
      "battery"
    ]
  }
}
```

### Compact WiFi Sparkline
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time"],
    "right": [
      {
        "name": "wifi",
        "sparkline": true,
        "sparkline_length": 6
      },
      "separator",
      "battery"
    ]
  }
}
```

### Full System Monitoring with Sparklines
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

### Mixed Traditional and Sparkline Display
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

### Compact System Monitoring
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time"],
    "right": [
      {
        "name": "cpu",
        "sparkline": true,
        "sparkline_length": 4
      },
      "space",
      {
        "name": "ram",
        "sparkline": true,
        "sparkline_length": 4
      },
      "space",
      {
        "name": "wifi",
        "sparkline": true,
        "sparkline_length": 4
      }
    ]
  }
}
```

### Custom Grouping
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

### Fine-tuned Spacing
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

### Minimal with Custom Spacing
```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["time", "space", "weather"],
    "right": ["wifi", "space", "battery"]
  }
}
```

## Spacer Customization

The configuration system provides two spacing components:

- **`separator`** - Visual separator (" | ") for creating distinct sections
- **`space`** - Single space character (" ") for fine-tuned spacing between components

You can use multiple `space` components in a row for larger gaps, or combine `space` and `separator` for custom layouts. For example, `["space", "separator", "space"]` would create " | " with extra spacing around the separator.

The first time you run catfood_bar, it will create a default configuration file at `~/.config/catfood/bar.json`. You can then edit this file to customize your bar layout.
