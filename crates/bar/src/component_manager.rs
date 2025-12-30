use crate::components::{
    Battery, Brightness, Cpu, ErrorIcon, KittyTabs, Ram, Separator, Space, Temperature, Time,
    Volume, Weather, Wifi, Windows, Workspaces,
};
use crate::config::{ComponentConfig, Config};
use crate::lua_component::{LuaComponent, LuaComponentRegistry};
use ratatui::{prelude::Stylize, text::Span};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Component {
    Workspaces(Workspaces),
    Windows(Windows),
    KittyTabs(KittyTabs),
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
        component_config: &ComponentConfig,
        lua_registry: Option<&LuaComponentRegistry>,
        config: &Config,
    ) -> color_eyre::Result<Self> {
        let component_type = component_config.name();
        match component_type {
            "workspaces" => Ok(Component::Workspaces(Workspaces::new())),
            "windows" => Ok(Component::Windows(Windows::new())),
            "kitty_tabs" => {
                let socket_path = component_config.socket_path();
                Ok(Component::KittyTabs(KittyTabs::with_config(socket_path)))
            }
            "time" => Ok(Component::Time(Time::with_config(
                config.day_start,
                config.night_start,
            ))),
            "weather" => Ok(Component::Weather(Weather::with_config(
                config.day_start,
                config.night_start,
            ))),
            "temperature" => Ok(Component::Temperature(Temperature::new())),
            "cpu" => {
                let sparkline = component_config.sparkline().unwrap_or(false);
                let sparkline_length = component_config.sparkline_length().unwrap_or(10);
                let sparkline_update_freq = component_config.sparkline_update_freq().unwrap_or(3);
                let sparkline_logarithmic =
                    component_config.sparkline_logarithmic().unwrap_or(false); // Linear by default
                Ok(Component::Cpu(Cpu::with_config(
                    sparkline,
                    sparkline_length,
                    sparkline_update_freq,
                    sparkline_logarithmic,
                )))
            }
            "ram" => {
                let sparkline = component_config.sparkline().unwrap_or(false);
                let sparkline_length = component_config.sparkline_length().unwrap_or(10);
                let sparkline_update_freq = component_config.sparkline_update_freq().unwrap_or(2);
                let sparkline_logarithmic =
                    component_config.sparkline_logarithmic().unwrap_or(false); // Linear by default
                Ok(Component::Ram(Ram::with_config(
                    sparkline,
                    sparkline_length,
                    sparkline_update_freq,
                    sparkline_logarithmic,
                )))
            }
            "wifi" => {
                let sparkline = component_config.sparkline().unwrap_or(false);
                let sparkline_length = component_config.sparkline_length().unwrap_or(10);
                let sparkline_update_freq = component_config.sparkline_update_freq().unwrap_or(2);
                let sparkline_logarithmic =
                    component_config.sparkline_logarithmic().unwrap_or(true); // Logarithmic by default for WiFi
                Ok(Component::Wifi(Wifi::with_config(
                    sparkline,
                    sparkline_length,
                    sparkline_update_freq,
                    sparkline_logarithmic,
                )))
            }
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
            Component::Windows(component) => {
                component.update();
                Ok(())
            }
            Component::KittyTabs(component) => {
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
            Component::Workspaces(component) => component.render_as_spans(colorize),
            Component::Windows(component) => component.render_as_spans(colorize),
            Component::KittyTabs(component) => component.render_as_spans(colorize),
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
                .join("catfood")
                .join("components");
        lua_registry.load_from_directory(config_dir.to_str().unwrap())?;

        let mut components = HashMap::new();

        // Create all components (unknown ones become error icons)
        for component_config in &config.bars.left {
            let component = Component::new(component_config, Some(&lua_registry), &config)?;
            components.insert(component_config.name().to_string(), component);
        }

        for component_config in &config.bars.middle {
            if !components.contains_key(component_config.name()) {
                let component = Component::new(component_config, Some(&lua_registry), &config)?;
                components.insert(component_config.name().to_string(), component);
            }
        }

        for component_config in &config.bars.right {
            if !components.contains_key(component_config.name()) {
                let component = Component::new(component_config, Some(&lua_registry), &config)?;
                components.insert(component_config.name().to_string(), component);
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
        if let Some(component_configs) = self.config.get_components_for_bar(bar) {
            component_configs
                .iter()
                .filter_map(|config| self.components.get(config.name()))
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
                .join("catfood")
                .join("components");
        self.lua_registry = LuaComponentRegistry::new();
        self.lua_registry
            .load_from_directory(config_dir.to_str().unwrap())?;

        let mut components = HashMap::new();

        // Create all components (unknown ones become error icons)
        for component_config in &new_config.bars.left {
            let component =
                Component::new(component_config, Some(&self.lua_registry), &new_config)?;
            components.insert(component_config.name().to_string(), component);
        }

        for component_config in &new_config.bars.middle {
            if !components.contains_key(component_config.name()) {
                let component =
                    Component::new(component_config, Some(&self.lua_registry), &new_config)?;
                components.insert(component_config.name().to_string(), component);
            }
        }

        for component_config in &new_config.bars.right {
            if !components.contains_key(component_config.name()) {
                let component =
                    Component::new(component_config, Some(&self.lua_registry), &new_config)?;
                components.insert(component_config.name().to_string(), component);
            }
        }

        self.config = new_config;
        self.components = components;
        Ok(())
    }
}
