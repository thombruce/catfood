use chrono::{Local, Timelike};
use ratatui::{prelude::Stylize, style::Color, text::Span};

#[derive(Debug, Clone)]
pub struct Time {
    pub time_string: String,
    pub cached_span_content: String,
    day_start: u8,
    night_start: u8,
}

impl Default for Time {
    fn default() -> Self {
        Self::with_config(6, 18)
    }
}

impl Time {
    pub fn new() -> Self {
        Self::with_config(6, 18)
    }

    pub fn with_config(day_start: u8, night_start: u8) -> Self {
        let time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            time_string: time_string.clone(),
            cached_span_content: time_string,
            day_start,
            night_start,
        }
    }

    pub fn update(&mut self) {
        self.time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.cached_span_content = self.time_string.clone();
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(&self.cached_span_content);
        if colorize {
            let hour = Local::now().hour();
            let color = if hour >= self.day_start as u32 && hour < self.night_start as u32 {
                Color::Yellow // Daytime: Yellow
            } else {
                Color::Magenta // Nighttime: Purple
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }
}
