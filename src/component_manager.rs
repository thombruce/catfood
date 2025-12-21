use crate::components::{
    Battery, Brightness, Cpu, ErrorIcon, Ram, Separator, Space, Temperature, Time, Volume,
    Weather, Wifi, Workspaces,
};
use crate::config::Config;
use crate::lua_component::{LuaComponent, LuaComponentRegistry};
use ratatui::{prelude::Stylize, text::Span};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Component {
    Workspaces(Workspaces),
    Time(Time),
    Weather(Weather),
    Temperature(Temperature),
    Cpu(Cpu),
    Ram(Ram),
    Wifi(Wifi),
    Brightness(Brightness),
    Volume(Volume),
    Battery(Battery),
    Separator(Separator),
    Space(Space),
    ErrorIcon(ErrorIcon),
    Lua(LuaComponent),
}

impl Component {
    pub fn new(
        component_type: &str,
        lua_registry: Option<&LuaComponentRegistry>,
    ) -> color_eyre::Result<Self> {
        match component_type {
            "workspaces" => Ok(Component::Workspaces(Workspaces::new())),
            "time" => Ok(Component::Time(Time::new())),
            "weather" => Ok(Component::Weather(Weather::new())),
            "temperature" => Ok(Component::Temperature(Temperature::new())),
            "cpu" => Ok(Component::Cpu(Cpu::new())),
            "ram" => Ok(Component::Ram(Ram::new())),
            "wifi" => Ok(Component::Wifi(Wifi::new())),
            "brightness" => Ok(Component::Brightness(Brightness::new())),
            "volume" => Ok(Component::Volume(Volume::new())),
            "battery" => Ok(Component::Battery(Battery::new()?)),
            "separator" => Ok(Component::Separator(Separator::new())),
            "space" => Ok(Component::Space(Space::new())),
            _ => {
                // Try to load as Lua component
                if let Some(registry) = lua_registry
                    && let Some(lua_component) = registry.get_component(component_type)
                {
                    return Ok(Component::Lua(lua_component.clone()));
                }
                Ok(Component::ErrorIcon(ErrorIcon::new()))
            }
        }
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        match self {
            Component::Workspaces(component) => {
                component.update();
                Ok(())
            }
            Component::Time(component) => {
                component.update();
                Ok(())
            }
            Component::Weather(component) => {
                component.update();
                Ok(())
            }
            Component::Temperature(component) => {
                component.update();
                Ok(())
            }
            Component::Cpu(component) => {
                component.update();
                Ok(())
            }
            Component::Ram(component) => {
                component.update();
                Ok(())
            }
            Component::Wifi(component) => {
                component.update();
                Ok(())
            }
            Component::Brightness(component) => {
                component.update();
                Ok(())
            }
            Component::Volume(component) => {
                component.update();
                Ok(())
            }
            Component::Battery(component) => {
                component.update()?;
                Ok(())
            }
            Component::Separator(_component) => {
                // Separator doesn't need updates
                Ok(())
            }
            Component::Space(_component) => {
                // Space doesn't need updates
                Ok(())
            }
            Component::ErrorIcon(_component) => {
                // ErrorIcon doesn't need updates
                Ok(())
            }
            Component::Lua(component) => {
                component.update()?;
                Ok(())
            }
        }
    }

    pub fn render_as_spans_with_colorize(&self, colorize: bool) -> Vec<Span<'_>> {
        match self {
            Component::Workspaces(component) => component.render(),
            Component::Time(component) => component.render_as_spans(colorize),
            Component::Weather(component) => component.render_as_spans(colorize),
            Component::Temperature(component) => component.render_as_spans(colorize),
            Component::Cpu(component) => component.render_as_spans(colorize),
            Component::Ram(component) => component.render_as_spans(colorize),
            Component::Wifi(component) => component.render_as_spans(colorize),
            Component::Brightness(component) => component.render_as_spans(colorize),
            Component::Volume(component) => component.render_as_spans(colorize),
            Component::Battery(component) => component.render_as_spans(colorize),
            Component::Separator(component) => vec![Span::raw(component.render())],
            Component::Space(component) => vec![Span::raw(component.render())],
            Component::ErrorIcon(component) => component.render_as_spans(),
            Component::Lua(component) => component.render_as_spans_with_colorize(colorize),
        }
    }

    pub fn is_muted(&self) -> bool {
        match self {
            Component::Volume(component) => component.is_muted,
            _ => false,
        }
    }

    pub fn render_as_spans_with_muting_and_colorize(&self, colorize: bool) -> Vec<Span<'_>> {
        let spans = self.render_as_spans_with_colorize(colorize);
        if self.is_muted() {
            spans
                .into_iter()
                .map(|span| span.fg(ratatui::style::Color::DarkGray))
                .collect()
        } else {
            spans
        }
    }
}

#[derive(Debug)]
pub struct ComponentManager {
    components: HashMap<String, Component>,
    config: Config,
    lua_registry: LuaComponentRegistry,
}

impl ComponentManager {
    pub fn new() -> color_eyre::Result<Self> {
        let config = Config::load()?;
        let mut lua_registry = LuaComponentRegistry::new();

        // Load Lua components from config directory
        let config_dir =
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".config")
                .join("catfoodBar")
                .join("components");
        lua_registry.load_from_directory(config_dir.to_str().unwrap())?;

        let mut components = HashMap::new();

        // Create all components (unknown ones become error icons)
        for component_name in &config.bars.left {
            let component = Component::new(component_name, Some(&lua_registry))?;
            components.insert(component_name.clone(), component);
        }

        for component_name in &config.bars.middle {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name, Some(&lua_registry))?;
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &config.bars.right {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name, Some(&lua_registry))?;
                components.insert(component_name.clone(), component);
            }
        }

        Ok(Self {
            components,
            config,
            lua_registry,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        // Update built-in components
        for component in self.components.values_mut() {
            component.update()?;
        }

        Ok(())
    }

    pub fn get_bar_components(&self, bar: &str) -> Vec<&Component> {
        if let Some(component_names) = self.config.get_components_for_bar(bar) {
            component_names
                .iter()
                .filter_map(|name| self.components.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_colorize(&self) -> bool {
        self.config.colorize
    }

    pub fn reload(&mut self) -> color_eyre::Result<()> {
        let new_config = self.config.reload()?;

        // Reload Lua components
        let config_dir =
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".config")
                .join("catfoodBar")
                .join("components");
        self.lua_registry = LuaComponentRegistry::new();
        self.lua_registry
            .load_from_directory(config_dir.to_str().unwrap())?;

        let mut components = HashMap::new();

        // Create all components (unknown ones become error icons)
        for component_name in &new_config.bars.left {
            let component = Component::new(component_name, Some(&self.lua_registry))?;
            components.insert(component_name.clone(), component);
        }

        for component_name in &new_config.bars.middle {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name, Some(&self.lua_registry))?;
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &new_config.bars.right {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name, Some(&self.lua_registry))?;
                components.insert(component_name.clone(), component);
            }
        }

        self.config = new_config;
        self.components = components;
        Ok(())
    }
}
