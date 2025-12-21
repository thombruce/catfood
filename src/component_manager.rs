use crate::components::{
    Battery, Brightness, Cpu, ErrorIcon, Ram, Separator, Space, Temperature, Time, Volume, Vpn,
    Weather, Wifi, Workspaces,
};
use crate::config::Config;
use chrono::Timelike;
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
            Component::Time(component) => {
                let span = Span::raw(component.time_string.clone());
                if colorize {
                    let hour = chrono::Local::now().hour();
                    let color = if (6..18).contains(&hour) {
                        Color::Yellow // Daytime (6:00 - 17:59): Yellow
                    } else {
                        Color::Magenta // Nighttime (18:00 - 5:59): Purple
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Component::Weather(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    vec![span.fg(Color::White)]
                } else {
                    vec![span]
                }
            }
            Component::Temperature(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    let color = if let Ok(temp) = component.value.parse::<u32>() {
                        if temp >= 80 {
                            Color::Red // High temp: Red
                        } else {
                            Color::Yellow // Normal: Yellow
                        }
                    } else {
                        Color::Yellow
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Component::Cpu(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    let color = if let Ok(usage) = component.usage.parse::<u32>() {
                        if usage >= 90 {
                            Color::Red // High CPU usage: Red
                        } else {
                            Color::White // Normal: White
                        }
                    } else {
                        Color::White
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Component::Ram(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    let color = if let Ok(usage) = component.usage.parse::<u32>() {
                        if usage >= 90 {
                            Color::Red // High RAM usage: Red
                        } else {
                            Color::Green // Normal: Green
                        }
                    } else {
                        Color::Green
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Component::Wifi(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    let color = if component.status == "disconnected" {
                        Color::Red
                    } else {
                        Color::Blue
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Component::Vpn(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    let color = if component.status == "disconnected" {
                        Color::DarkGray
                    } else {
                        Color::Magenta
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
            Component::Brightness(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    vec![span.fg(Color::White)]
                } else {
                    vec![span]
                }
            }
            Component::Volume(component) => {
                if component.is_muted || !colorize {
                    vec![Span::raw(component.render())]
                } else {
                    vec![Span::raw(component.render()).fg(Color::White)]
                }
            }
            Component::Battery(component) => {
                let span = Span::raw(component.render());
                if colorize {
                    let color = if component.is_charging {
                        Color::Green
                    } else if let Ok(percentage) = component.percentage.parse::<u32>() {
                        if percentage <= 10 {
                            Color::Red // Very low: Red
                        } else if percentage <= 25 {
                            Color::Yellow // Low: Yellow/Amber  
                        } else {
                            Color::Green // Normal/High: Green
                        }
                    } else {
                        Color::Red
                    };
                    vec![span.fg(color)]
                } else {
                    vec![span]
                }
            }
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
