use crate::ClickArea;
use crate::component_manager::ComponentManager;
use ratatui::{Frame, prelude::Stylize, style::Color, text::Line, widgets::Paragraph};

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
    ) -> Vec<ClickArea> {
        let components = component_manager.get_bar_components("left");
        let colorize = component_manager.get_colorize();

        if components.is_empty() {
            return Vec::new();
        }

        let mut all_spans = Vec::new();
        let mut all_click_areas = Vec::new();
        let mut current_x = area.x;

        for component in components {
            let (spans, click_areas) =
                component.render_as_spans_with_bounds_and_colorize(colorize, current_x, area.y);

            // Calculate the width of these spans
            let spans_width: u16 = spans.iter().map(|s| s.content.len() as u16).sum();

            all_spans.extend(spans);
            all_click_areas.extend(click_areas);

            // Update current_x for next component
            current_x += spans_width;
        }

        let left_line = Line::from(all_spans);

        frame.render_widget(
            Paragraph::new(left_line).left_aligned().fg(Color::White),
            area,
        );

        all_click_areas
    }
}
