use super::sparkline::Sparkline;
use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::time::{Duration, Instant};
use sysinfo::{MemoryRefreshKind, RefreshKind};

#[derive(Debug)]
pub struct Ram {
    pub usage: String,
    cached_span_content: String,
    system: sysinfo::System,
    last_update: Instant,
    update_interval: Duration,
    sparkline: Sparkline,
}

impl Ram {
    pub fn with_config(
        sparkline: bool,
        sparkline_length: usize,
        sparkline_update_freq: u64,
    ) -> Self {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        let usage = "0".to_string();
        let sparkline = Sparkline::new(sparkline, sparkline_length);
        let cached_span_content = if sparkline.enabled {
            format!("󰍛 {}", sparkline.render_with_spaces())
        } else {
            format!("󰍛 {}%", usage)
        };

        Self {
            usage,
            cached_span_content,
            system,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(sparkline_update_freq),
            sparkline,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.system.refresh_memory();

            let mem_percent: u32 = (self.system.used_memory() as f64
                / self.system.total_memory() as f64
                * 100.0) as u32;
            self.usage = mem_percent.to_string();

            if self.sparkline.enabled {
                // Update sparkline data
                self.sparkline.update(mem_percent as u64);

                // Render sparkline
                self.cached_span_content = format!("󰍛 {}", self.sparkline.render());
            } else {
                self.cached_span_content = format!("󰍛 {}%", self.usage);
            }

            self.last_update = now;
        }
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(&self.cached_span_content);
        if colorize {
            let color = if let Ok(usage) = self.usage.parse::<u32>() {
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
}
