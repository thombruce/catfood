use regex::Regex;
use std::process::Command;
use crate::logging;

static BRIGHTNESS_REGEX: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\d+%").unwrap());

#[derive(Debug)]
pub struct Brightness {
    pub level: String,
}

impl Brightness {
    pub fn new() -> Self {
        Self {
            level: get_system_brightness().unwrap_or_default(),
        }
    }

    pub fn update(&mut self) {
        self.level = get_system_brightness().unwrap_or_default();
    }

    pub fn render(&self) -> String {
        format!("󰃠 {}", self.level)
    }
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

        logging::log_component_error("BRIGHTNESS", &format!("Failed to parse brightness from output: {}", brightness_str));
    } else {
        logging::log_component_error("BRIGHTNESS", 
            str::from_utf8(&output.stderr).unwrap_or("unknown error"));
    }

    None
}
