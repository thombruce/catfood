use crate::component_manager::ComponentManager;
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct MiddleBar;

impl MiddleBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self)
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        Ok(())
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        component_manager: &ComponentManager,
    ) {
        let components = component_manager.get_bar_components("middle");
        let colorize = component_manager.get_colorize();

        if components.is_empty() {
            return;
        }

        let spans: Vec<Span> = components
            .iter()
            .flat_map(|component| component.render_as_spans_with_colorize(colorize))
            .collect();

        let middle_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(middle_line).centered().fg(Color::White),
            area,
        );
    }
}
