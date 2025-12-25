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
    sparkline: bool,
    sparkline_length: usize,
    sparkline_data: Vec<u32>,
}

impl Ram {
    pub fn new() -> Self {
        Self::with_config(false, 10, 2)
    }

    pub fn with_config(
        sparkline: bool,
        sparkline_length: usize,
        sparkline_update_freq: u64,
    ) -> Self {
        let system = sysinfo::System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        let usage = "0".to_string();
        let cached_span_content = if sparkline {
            let sparkline_str = " ".repeat(sparkline_length);
            format!("󰍛 {}", sparkline_str)
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
            sparkline_length,
            sparkline_data: vec![0; sparkline_length],
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

            if self.sparkline {
                // Update sparkline data (shift left and add new value)
                self.sparkline_data.remove(0);
                self.sparkline_data.push(mem_percent);

                // Render sparkline
                let sparkline_str = self.render_sparkline();
                self.cached_span_content = format!("󰍛 {}", sparkline_str);
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

    fn render_sparkline(&self) -> String {
        let max_value = self.sparkline_data.iter().max().unwrap_or(&1);
        if *max_value == 0 {
            return " ".repeat(self.sparkline_length);
        }

        let bars = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        let mut result = String::new();

        for &value in &self.sparkline_data {
            if value == 0 {
                result.push(' ');
            } else {
                let index = ((value as f64 / *max_value as f64) * (bars.len() - 1) as f64) as usize;
                result.push(bars[index.min(bars.len() - 1)].chars().next().unwrap());
            }
        }

        result
    }
}
