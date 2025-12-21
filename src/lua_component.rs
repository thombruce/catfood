use mlua::{Function, Lua, Table, Value};
use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LuaComponent {
    name: String,
    #[allow(dead_code)]
    lua: Lua,
    update_fn: Option<Function>,
    render_fn: Function,
    #[allow(dead_code)]
    config: Option<Table>,
}

impl LuaComponent {
    pub fn new(name: String, script_path: &str) -> color_eyre::Result<Self> {
        let lua = Lua::new();

        // Load the Lua script
        let script = std::fs::read_to_string(script_path)?;

        // Execute the script and get the returned component table
        let component_table: Table = lua
            .load(&script)
            .eval()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to load Lua script: {}", e))?;

        // Extract functions and config
        let update_fn: Option<Function> = component_table.get("update").ok();
        let render_fn: Function = component_table
            .get("render")
            .map_err(|e| color_eyre::eyre::eyre!("Failed to get render function: {}", e))?;
        let config: Option<Table> = component_table.get("config").ok();

        Ok(Self {
            name,
            lua,
            update_fn,
            render_fn,
            config,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        if let Some(ref update_fn) = self.update_fn {
            update_fn
                .call::<()>(())
                .map_err(|e| color_eyre::eyre::eyre!("Failed to call update function: {}", e))?;
        }
        Ok(())
    }

    pub fn render_as_spans_with_colorize(&self, colorize: bool) -> Vec<Span<'_>> {
        // Try to call render function that returns (text, color)
        match self.render_fn.call::<Value>((colorize,)) {
            Ok(Value::String(text)) => {
                let text_str = text.to_string_lossy();
                vec![Span::raw(text_str.to_string())]
            }
            Ok(Value::Table(table)) => {
                let mut text = "error".to_string();
                let mut color = None;

                if let Ok(val) = table.get(1)
                    && let Value::String(s) = val
                {
                    text = s.to_string_lossy().to_string();
                }

                if let Ok(val) = table.get(2)
                    && let Value::String(s) = val
                {
                    color = Some(s.to_string_lossy().to_string());
                }

                let span = Span::raw(text);
                if let Some(color_name) = color {
                    let color = self.parse_color(&color_name);
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Ok(_) => {
                vec![Span::raw(format!("❌ {}", self.name))]
            }
            Err(_) => {
                vec![Span::raw(format!("❌ {}", self.name))]
            }
        }
    }

    fn parse_color(&self, color_name: &str) -> Color {
        match color_name.to_lowercase().as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "black" => Color::Black,
            "gray" | "grey" => Color::Gray,
            "dark_red" => Color::DarkGray,
            "dark_green" => Color::DarkGray,
            "dark_yellow" => Color::DarkGray,
            "dark_blue" => Color::DarkGray,
            "dark_magenta" => Color::DarkGray,
            "dark_cyan" => Color::DarkGray,
            _ => Color::White,
        }
    }
}

#[derive(Debug)]
pub struct LuaComponentRegistry {
    components: HashMap<String, LuaComponent>,
}

impl LuaComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn load_component(&mut self, name: &str, script_path: &str) -> color_eyre::Result<()> {
        let component = LuaComponent::new(name.to_string(), script_path)?;
        self.components.insert(name.to_string(), component);
        Ok(())
    }

    pub fn get_component(&self, name: &str) -> Option<&LuaComponent> {
        self.components.get(name)
    }

    pub fn load_from_directory(&mut self, dir_path: &str) -> color_eyre::Result<()> {
        if !std::path::Path::new(dir_path).exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("lua")
                && let Some(name) = path.file_stem().and_then(|s| s.to_str())
            {
                self.load_component(name, path.to_str().unwrap())?;
            }
        }

        Ok(())
    }
}
