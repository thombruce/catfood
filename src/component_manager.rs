use crate::components::{
    Battery, Brightness, Cpu, ErrorIcon, Ram, Separator, Space, Temperature, Time, Volume, Vpn,
    Weather, Wifi, Workspaces,
};
use crate::config::Config;
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
    Vpn(Vpn),
    Brightness(Brightness),
    Volume(Volume),
    Battery(Battery),
    Separator(Separator),
    Space(Space),
    ErrorIcon(ErrorIcon),
}

impl Component {
    pub fn new(component_type: &str) -> color_eyre::Result<Self> {
        match component_type {
            "workspaces" => Ok(Component::Workspaces(Workspaces::new())),
            "time" => Ok(Component::Time(Time::new())),
            "weather" => Ok(Component::Weather(Weather::new())),
            "temperature" => Ok(Component::Temperature(Temperature::new())),
            "cpu" => Ok(Component::Cpu(Cpu::new())),
            "ram" => Ok(Component::Ram(Ram::new())),
            "wifi" => Ok(Component::Wifi(Wifi::new())),
            "vpn" => Ok(Component::Vpn(Vpn::new())),
            "brightness" => Ok(Component::Brightness(Brightness::new())),
            "volume" => Ok(Component::Volume(Volume::new())),
            "battery" => Ok(Component::Battery(Battery::new()?)),
            "separator" => Ok(Component::Separator(Separator::new())),
            "space" => Ok(Component::Space(Space::new())),
            _ => Ok(Component::ErrorIcon(ErrorIcon::new())),
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
            Component::Vpn(component) => {
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
            Component::Vpn(component) => component.render_as_spans(colorize),
            Component::Brightness(component) => component.render_as_spans(colorize),
            Component::Volume(component) => component.render_as_spans(colorize),
            Component::Battery(component) => component.render_as_spans(colorize),
            Component::Separator(component) => vec![Span::raw(component.render())],
            Component::Space(component) => vec![Span::raw(component.render())],
            Component::ErrorIcon(component) => component.render_as_spans(),
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
}

impl ComponentManager {
    pub fn new() -> color_eyre::Result<Self> {
        let config = Config::load()?;
        let mut components = HashMap::new();

        // Create all components (unknown ones become error icons)
        for component_name in &config.bars.left {
            let component = Component::new(component_name)?;
            components.insert(component_name.clone(), component);
        }

        for component_name in &config.bars.middle {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name)?;
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &config.bars.right {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name)?;
                components.insert(component_name.clone(), component);
            }
        }

        Ok(Self { components, config })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
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
        let mut components = HashMap::new();

        // Create all components (unknown ones become error icons)
        for component_name in &new_config.bars.left {
            let component = Component::new(component_name)?;
            components.insert(component_name.clone(), component);
        }

        for component_name in &new_config.bars.middle {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name)?;
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &new_config.bars.right {
            if !components.contains_key(component_name) {
                let component = Component::new(component_name)?;
                components.insert(component_name.clone(), component);
            }
        }

        self.config = new_config;
        self.components = components;
        Ok(())
    }
}
