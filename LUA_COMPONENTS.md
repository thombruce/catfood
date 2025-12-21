# Lua Components Extensibility

catfoodBar now supports custom Lua components that can be placed in any of the three bar areas (left, middle, right) alongside the built-in components.

## How it Works

1. **Component Loading**: Lua components are automatically loaded from `~/.config/catfoodBar/components/`
2. **Configuration**: Components are referenced by name in `config.json` just like built-in components
3. **Integration**: Lua components are treated the same as built-in components for updates and rendering

## Lua Component Structure

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

## Available Colors

Lua components can use the following color names:
- `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `black`
- `gray`/`grey`, `dark_red`, `dark_green`, `dark_yellow`, etc.

## Example Components

### Simple Clock Component

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

### System Uptime Component

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

## Configuration

Add Lua components to your `config.json`:

```json
{
  "bars": {
    "left": ["workspaces"],
    "middle": ["lua_clock", "separator", "weather"],
    "right": ["uptime", "separator", "battery"]
  },
  "colorize": true
}
```

## Installation

1. Create the components directory:
   ```bash
   mkdir -p ~/.config/catfoodBar/components
   ```

2. Add your Lua component files (`.lua` extension)
3. Reference them in `config.json` by filename (without extension)
4. Restart catfoodBar or wait for automatic config reload

## Error Handling

- If a Lua component fails to load or has errors, it will display as `❌ component_name`
- Built-in components are unaffected by Lua component failures
- Check application logs for detailed error information

## Limitations

- Lua components run in the same process as the main application
- Long-running operations in `update()` may affect UI responsiveness
- No direct file system or network access beyond standard Lua libraries (can be extended if needed)
- Component state is not persisted across restarts