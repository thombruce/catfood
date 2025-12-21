use crate::components::{
    Battery, Brightness, Cpu, Ram, Separator, Space, Temperature, Time, Volume, Vpn, Weather, Wifi,
    Workspaces,
};
use crate::config::Config;
use ratatui::{prelude::Stylize, style::Color, text::Span};
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
            _ => Err(color_eyre::eyre::eyre!(
                "Unknown component type: {}",
                component_type
            )),
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
        }
    }

    pub fn render_as_spans(&self) -> Vec<Span<'_>> {
        match self {
            Component::Workspaces(component) => component.render(),
            Component::Time(component) => vec![Span::raw(component.time_string.clone())],
            Component::Weather(component) => vec![Span::raw(component.render())],
            Component::Temperature(component) => vec![Span::raw(component.render())],
            Component::Cpu(component) => vec![Span::raw(component.render())],
            Component::Ram(component) => vec![Span::raw(component.render())],
            Component::Wifi(component) => vec![Span::raw(component.render())],
            Component::Vpn(component) => vec![Span::raw(component.render())],
            Component::Brightness(component) => vec![Span::raw(component.render())],
            Component::Volume(component) => vec![Span::raw(component.render())],
            Component::Battery(component) => vec![Span::raw(component.render())],
            Component::Separator(component) => vec![Span::raw(component.render())],
            Component::Space(component) => vec![Span::raw(component.render())],
        }
    }



    pub fn is_muted(&self) -> bool {
        match self {
            Component::Volume(component) => component.is_muted,
            _ => false,
        }
    }

    pub fn render_as_spans_with_muting(&self) -> Vec<Span<'_>> {
        let spans = self.render_as_spans();
        if self.is_muted() {
            spans
                .into_iter()
                .map(|span| span.fg(Color::DarkGray))
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
        let mut unknown_components = Vec::new();

        // Collect all component names for validation
        let all_component_names: Vec<&String> = config.bars.left
            .iter()
            .chain(config.bars.middle.iter())
            .chain(config.bars.right.iter())
            .collect();

        // Validate component names first
        for component_name in &all_component_names {
            if Component::new(component_name).is_err() {
                unknown_components.push((*component_name).clone());
            }
        }

        // Warn about unknown components
        if !unknown_components.is_empty() {
            eprintln!("Warning: Unknown components found in configuration: {:?}", unknown_components);
            eprintln!("Available components: workspaces, time, weather, temperature, cpu, ram, wifi, vpn, brightness, volume, battery, separator, space");
        }

        // Create valid components
        for component_name in &config.bars.left {
            if let Ok(component) = Component::new(component_name) {
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &config.bars.middle {
            if !components.contains_key(component_name)
                && let Ok(component) = Component::new(component_name)
            {
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &config.bars.right {
            if !components.contains_key(component_name)
                && let Ok(component) = Component::new(component_name)
            {
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

    pub fn reload(&mut self) -> color_eyre::Result<()> {
        let new_config = self.config.reload()?;
        let mut components = HashMap::new();
        let mut unknown_components = Vec::new();

        // Collect all component names for validation
        let all_component_names: Vec<&String> = new_config.bars.left
            .iter()
            .chain(new_config.bars.middle.iter())
            .chain(new_config.bars.right.iter())
            .collect();

        // Validate component names first
        for component_name in &all_component_names {
            if Component::new(component_name).is_err() {
                unknown_components.push((*component_name).clone());
            }
        }

        // Warn about unknown components
        if !unknown_components.is_empty() {
            eprintln!("Warning: Unknown components found in configuration: {:?}", unknown_components);
            eprintln!("Available components: workspaces, time, weather, temperature, cpu, ram, wifi, vpn, brightness, volume, battery, separator, space");
        }

        // Create valid components
        for component_name in &new_config.bars.left {
            if let Ok(component) = Component::new(component_name) {
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &new_config.bars.middle {
            if !components.contains_key(component_name)
                && let Ok(component) = Component::new(component_name)
            {
                components.insert(component_name.clone(), component);
            }
        }

        for component_name in &new_config.bars.right {
            if !components.contains_key(component_name)
                && let Ok(component) = Component::new(component_name)
            {
                components.insert(component_name.clone(), component);
            }
        }

        self.config = new_config;
        self.components = components;
        Ok(())
    }
}
