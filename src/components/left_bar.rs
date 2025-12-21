use crate::component_manager::ComponentManager;
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct LeftBar;

impl LeftBar {
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
        let components = component_manager.get_bar_components("left");
        let colorize = component_manager.get_colorize();

        if components.is_empty() {
            return;
        }

        let spans: Vec<Span> = components
            .iter()
            .flat_map(|component| component.render_as_spans_with_colorize(colorize))
            .collect();

        let left_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(left_line).left_aligned().fg(Color::White),
            area,
        );
    }
}
