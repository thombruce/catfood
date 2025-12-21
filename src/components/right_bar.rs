use crate::components::{Battery, Brightness, Cpu, Ram, Temperature, Volume, Vpn, Wifi};
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct RightBar {
    temperature: Temperature,
    cpu: Cpu,
    ram: Ram,
    wifi: Wifi,
    vpn: Vpn,
    brightness: Brightness,
    volume: Volume,
    battery: Battery,
}

impl RightBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            temperature: Temperature::new(),
            cpu: Cpu::new(),
            ram: Ram::new(),
            wifi: Wifi::new(),
            vpn: Vpn::new(),
            brightness: Brightness::new(),
            volume: Volume::new(),
            battery: Battery::new()?,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.temperature.update();
        self.cpu.update();
        self.ram.update();
        self.wifi.update();
        self.vpn.update();
        self.brightness.update();
        self.volume.update();
        self.battery.update()?;
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let spans = vec![
            Span::raw(" "),
            Span::raw(self.temperature.value.clone() + "°C"),
            Span::raw(" "),
            Span::raw("󰻠 "),
            Span::raw(self.cpu.usage.clone() + "%"),
            Span::raw(" "),
            Span::raw("󰍛 "),
            Span::raw(self.ram.usage.clone() + "%"),
            Span::raw(" | "),
            Span::raw(if self.wifi.status == "connected" {
                "󰤨 "
            } else {
                "󰤮 "
            }),
            Span::raw(
                if self.wifi.status == "connected" && !self.wifi.network.is_empty() {
                    &self.wifi.network
                } else {
                    "Off"
                }
                .to_string(),
            ),
            Span::raw(" "),
            Span::raw(" "),
            Span::raw(self.vpn.short.clone()), // Add `+ " "` if we include the section below
            // Span::raw(
            //     if self.vpn.status == "connected" && !self.vpn.country.is_empty() {
            //         &self.vpn.country
            //     } else {
            //         "Off"
            //     }
            //     .to_string(),
            // ),
            Span::raw(" | "),
            Span::raw("󰃠 "),
            Span::raw(self.brightness.level.clone()),
            Span::raw(" "),
            if self.volume.is_muted {
                Span::raw("󰝟 ").fg(Color::DarkGray)
            } else {
                Span::raw("󰕾 ")
            },
            if self.volume.is_muted {
                Span::raw(self.volume.level.clone() + "%").fg(Color::DarkGray)
            } else {
                Span::raw(self.volume.level.clone() + "%")
            },
            Span::raw(" | "),
            Span::raw(if self.battery.is_charging {
                "󰂄 "
            } else {
                "󰁹 "
            }),
            Span::raw(self.battery.percentage.clone() + "%"),
        ];

        let right_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(right_line).right_aligned().fg(Color::White),
            area,
        );
    }
}
