use crate::time_utils;
use chrono::Local;
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
        Self::with_config(
            time_utils::default_day_start(),
            time_utils::default_night_start(),
        )
    }
}

impl Time {
    pub fn new() -> Self {
        Self::with_config(
            time_utils::default_day_start(),
            time_utils::default_night_start(),
        )
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
            let color = time_utils::get_time_based_color(
                Color::Yellow,  // Daytime: Yellow
                Color::Magenta, // Nighttime: Purple
                self.day_start,
                self.night_start,
            );
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }
}
