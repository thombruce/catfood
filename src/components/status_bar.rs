use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use regex::Regex;
use std::{io, process::Command};

static BRIGHTNESS_REGEX: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\d+%").unwrap());

#[derive(Debug)]
pub struct StatusBar {
    pub volume: String,
    pub brightness: String,
    pub bat_percent: String,
    battery_manager: battery::Manager,
    battery: battery::Battery,
}

impl StatusBar {
    pub fn new() -> color_eyre::Result<Self> {
        let manager = battery::Manager::new()?;
        let battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e.into());
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(io::Error::from(io::ErrorKind::NotFound).into());
            }
        };

        Ok(Self {
            volume: get_system_volume().unwrap_or(0).to_string(),
            brightness: get_system_brightness().unwrap_or_default(),
            bat_percent: ((battery.state_of_charge().value * 100.0) as i32).to_string(),
            battery_manager: manager,
            battery,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.volume = get_system_volume().unwrap_or(0).to_string();
        self.brightness = get_system_brightness().unwrap_or_default();
        self.battery_manager.refresh(&mut self.battery)?;
        self.bat_percent = ((self.battery.state_of_charge().value * 100.0) as i32).to_string();
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let brightness_icon = Span::raw("󰃠 ".to_owned());
        let brightness_span = Span::raw(self.brightness.clone());

        let vol_icon = Span::raw("󰕾 ".to_owned());
        let vol_span = Span::raw(self.volume.clone() + "%");

        let bat_icon = Span::raw("󰁹 ".to_owned());
        let bat_span = Span::raw(self.bat_percent.clone() + "%");

        let sep_span = Span::raw(" | ");
        let space_span = Span::raw(" ");

        let status_line = Line::from(vec![
            sep_span.clone(),
            brightness_icon,
            brightness_span,
            space_span.clone(),
            vol_icon,
            vol_span,
            sep_span.clone(),
            bat_icon,
            bat_span,
            space_span.clone(),
        ]);

        frame.render_widget(
            Paragraph::new(status_line).right_aligned().fg(Color::White),
            area,
        );
    }
}

fn get_system_volume() -> Option<i32> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .expect("failed to get volume");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let parts: Vec<&str> = stdout.trim().split_whitespace().collect();

        if let Ok(volume) = parts[1].parse::<f32>() {
            return Some((volume * 100.0) as i32);
        }

        eprintln!("Failed to parse volume from output: {}", stdout);
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    Some(0)
}

fn get_system_brightness() -> Option<String> {
    let output = Command::new("brightnessctl")
        .output()
        .expect("failed to get brightness");

    if output.status.success() {
        let brightness_str = str::from_utf8(&output.stdout).unwrap();

        let re = &BRIGHTNESS_REGEX;

        if let Some(brightness) = re.find(brightness_str).map(|m| m.as_str()) {
            return Some(brightness.to_string());
        }

        eprintln!("Failed to parse brightness from output: {}", brightness_str);
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    None
}

