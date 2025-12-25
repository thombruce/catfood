# CatfoodBar Configuration Example

This configuration file allows you to customize which components appear in each bar section.

## Location
`~/.config/catfood/bar.json`

## Available Components
- `workspaces` - Hyprland workspaces
- `time` - Current date and time
- `weather` - Weather information
- `temperature` - CPU temperature
- `cpu` - CPU usage
- `ram` - Memory usage
- `wifi` - WiFi connection status
- `brightness` - Screen brightness
- `volume` - Volume level
- `battery` - Battery status
- `separator` - Visual separator (" | ") for creating custom sections
- `space` - Single space character (" ") for fine-tuned spacing

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

#### WiFi Component Options
- **`name`** (required): Component name, must be "wifi"
- **`sparkline`** (optional, default: false): Enable sparkline mode to show network usage over time
- **`sparkline_length`** (optional, default: 10): Length of the sparkline in characters
- **`sparkline_update_freq`** (optional, default: 2): Update frequency in seconds for the sparkline

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
