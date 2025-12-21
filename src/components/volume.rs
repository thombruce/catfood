use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::process::Command;

use crate::logging;

#[derive(Debug)]
pub struct Volume {
    pub level: String,
    pub is_muted: bool,
}

impl Volume {
    pub fn new() -> Self {
        let (level, is_muted) = get_system_volume().unwrap_or((0, false));
        Self {
            level: level.to_string(),
            is_muted,
        }
    }

    pub fn update(&mut self) {
        let (level, is_muted) = get_system_volume().unwrap_or((0, false));
        self.level = level.to_string();
        self.is_muted = is_muted;
    }

    pub fn render(&self) -> String {
        let icon = if self.is_muted { "󰝟" } else { "󰕾" };
        format!("{} {}%", icon, self.level)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        if self.is_muted || !colorize {
            vec![Span::raw(self.render())]
        } else {
            vec![Span::raw(self.render()).fg(Color::White)]
        }
    }
}

fn get_system_volume() -> Option<(i32, bool)> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .expect("failed to get volume");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let is_muted = stdout.contains("[MUTED]");
        let parts: Vec<&str> = stdout.split_whitespace().collect();

        if let Ok(volume) = parts[1].parse::<f32>() {
            return Some(((volume * 100.0) as i32, is_muted));
        }

        logging::log_component_error(
            "VOLUME",
            &format!("Failed to parse volume from output: {}", stdout),
        );
    } else {
        logging::log_component_error(
            "VOLUME",
            str::from_utf8(&output.stderr).unwrap_or("unknown error"),
        );
    }

    Some((0, false))
}
