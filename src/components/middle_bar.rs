use crate::components::{Time, Weather};
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct MiddleBar {
    time: Time,
    weather: Weather,
}

impl MiddleBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            time: Time::new(),
            weather: Weather::new(),
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.time.update();
        self.weather.update();
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let spans = vec![
            Span::raw(self.time.time_string.clone()),
            Span::raw(" | "),
            Span::raw(self.weather.get_weather_data().icon.clone() + " "),
            Span::raw(self.weather.get_weather_data().temperature.clone() + "°C"),
        ];

        let middle_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(middle_line).centered().fg(Color::White),
            area,
        );
    }
}
