use chrono::{Local, Timelike};
use ratatui::{prelude::Stylize, style::Color, text::Span};

#[derive(Debug, Default, Clone)]
pub struct Time {
    pub time_string: String,
}

impl Time {
    pub fn new() -> Self {
        Self {
            time_string: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn update(&mut self) {
        self.time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.time_string.clone());
        if colorize {
            let hour = Local::now().hour();
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
}
