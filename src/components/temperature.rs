use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::time::{Duration, Instant};
use sysinfo::Components;

#[derive(Debug)]
pub struct Temperature {
    pub value: String,
    components: Components,
    last_update: Instant,
    update_interval: Duration,
}

impl Temperature {
    pub fn new() -> Self {
        let components = Components::new();

        Self {
            value: "0".to_string(),
            components,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(5),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.components.refresh(true);

            if let Some(component) = self.components.iter().find(|c| {
                c.label().to_lowercase().contains("cpu")
                    || c.label().to_lowercase().contains("core")
                    || c.label().to_lowercase().contains("package")
            }) && let Some(temp) = component.temperature()
            {
                self.value = format!("{:.0}", temp);
            }

            self.last_update = now;
        }
    }

    pub fn render(&self) -> String {
        format!(" {}°C", self.value)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.render());
        if colorize {
            let color = if let Ok(temp) = self.value.parse::<u32>() {
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
}
