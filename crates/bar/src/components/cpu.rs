use super::sparkline::Sparkline;
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
    sparkline: Sparkline,
}

impl Cpu {
    pub fn with_config(
        sparkline: bool,
        sparkline_length: usize,
        sparkline_update_freq: u64,
    ) -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        let usage = "0".to_string();
        let sparkline = Sparkline::new(sparkline, sparkline_length);
        let cached_span_content = if sparkline.enabled {
            format!("󰻠 {}", sparkline.render_with_spaces())
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

            if self.sparkline.enabled {
                // Update sparkline data
                self.sparkline.update(avg as u64);

                // Render sparkline
                self.cached_span_content = format!("󰻠 {}", self.sparkline.render());
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
}
