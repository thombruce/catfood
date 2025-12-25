use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Debug)]
pub struct Cpu {
    pub usage: String,
    cached_span_content: String,
    system: System,
    last_update: Instant,
    update_interval: Duration,
    sparkline: bool,
    sparkline_length: usize,
    sparkline_data: Vec<u32>,
}

impl Cpu {
    pub fn new() -> Self {
        Self::with_config(false, 10, 3)
    }

    pub fn with_config(
        sparkline: bool,
        sparkline_length: usize,
        sparkline_update_freq: u64,
    ) -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        let usage = "0".to_string();
        let cached_span_content = if sparkline {
            let sparkline_str = " ".repeat(sparkline_length);
            format!("󰻠 {}", sparkline_str)
        } else {
            format!("󰻠 {}%", usage)
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
            self.system.refresh_cpu_all();

            let iter = self.system.cpus().iter();
            let count = iter.len() as f32;
            let sum = iter.fold(0.0, |acc, x| acc + x.cpu_usage());
            let avg: u32 = (sum / count) as u32;
            self.usage = avg.to_string();

            if self.sparkline {
                // Update sparkline data (shift left and add new value)
                self.sparkline_data.remove(0);
                self.sparkline_data.push(avg);

                // Render sparkline
                let sparkline_str = self.render_sparkline();
                self.cached_span_content = format!("󰻠 {}", sparkline_str);
            } else {
                self.cached_span_content = format!("󰻠 {}%", self.usage);
            }

            self.last_update = now;
        }
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(&self.cached_span_content);
        if colorize {
            let color = if let Ok(usage) = self.usage.parse::<u32>() {
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
