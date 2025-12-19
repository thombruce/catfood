use chrono::Local;
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::Span,
    widgets::Paragraph,
};

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

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let time_span = Span::raw(&self.time_string);
        frame.render_widget(
            Paragraph::new(time_span).centered().fg(Color::White),
            area,
        );
    }
}