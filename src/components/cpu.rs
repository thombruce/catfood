use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Debug)]
pub struct Cpu {
    pub usage: String,
    system: System,
    last_update: Instant,
    update_interval: Duration,
}

impl Cpu {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        Self {
            usage: "0".to_string(),
            system,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(3),
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

            self.last_update = now;
        }
    }

    pub fn render(&self) -> String {
        format!("󰻠 {}%", self.usage)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.render());
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
