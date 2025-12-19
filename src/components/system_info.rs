use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Debug)]
pub struct SystemInfo {
    pub cpu: String,
    pub ram: String,
    system: System,
}

impl SystemInfo {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        
        Self {
            cpu: "0".to_string(),
            ram: "0".to_string(),
            system,
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();

        let mem_percent: u32 =
            (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as u32;
        self.ram = mem_percent.to_string();

        let iter = self.system.cpus().iter();
        let count = iter.len() as f32;
        let sum = iter.fold(0.0, |acc, x| acc + x.cpu_usage());
        let avg: u32 = (sum / count) as u32;
        self.cpu = avg.to_string();
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let cpu_icon = Span::raw("󰻠 ".to_owned());
        let cpu_span = Span::raw(self.cpu.clone() + "%");
        let ram_icon = Span::raw("󰍛 ".to_owned());
        let ram_span = Span::raw(self.ram.clone() + "%");
        let space_span = Span::raw(" ");

        let system_line = Line::from(vec![
            cpu_icon,
            cpu_span,
            space_span.clone(),
            ram_icon,
            ram_span,
        ]);

        frame.render_widget(
            Paragraph::new(system_line)
                .right_aligned()
                .fg(Color::White),
            area,
        );
    }
}