use crate::logging;
use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Battery {
    pub percentage: String,
    pub is_charging: bool,
    battery_manager: battery::Manager,
    battery: battery::Battery,
    last_update: Instant,
    update_interval: Duration,
}

impl Battery {
    pub fn new() -> color_eyre::Result<Self> {
        let manager = battery::Manager::new()?;
        let battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                logging::log_component_error(
                    "BATTERY",
                    &format!("Unable to access battery information: {}", e),
                );
                return Err(e.into());
            }
            None => {
                logging::log_component_error("BATTERY", "Unable to find any batteries");
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
            }
        };

        let is_charging = matches!(battery.state(), battery::State::Charging);

        Ok(Self {
            percentage: ((battery.state_of_charge().value * 100.0) as i32).to_string(),
            is_charging,
            battery_manager: manager,
            battery,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(3),
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.battery_manager.refresh(&mut self.battery)?;
            self.percentage = ((self.battery.state_of_charge().value * 100.0) as i32).to_string();
            self.is_charging = matches!(self.battery.state(), battery::State::Charging);

            self.last_update = now;
        }
        Ok(())
    }

    pub fn render(&self) -> String {
        let icon = if self.is_charging { "󰂄" } else { "󰁹" };
        format!("{} {}%", icon, self.percentage)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.render());
        if colorize {
            let color = if self.is_charging {
                Color::Green
            } else if let Ok(percentage) = self.percentage.parse::<u32>() {
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
}
