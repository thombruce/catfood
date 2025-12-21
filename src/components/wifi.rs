use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Wifi {
    pub status: String,
    pub network: String,
    last_update: Instant,
    update_interval: Duration,
}

impl Wifi {
    pub fn new() -> Self {
        let (status, network) =
            get_wifi_status().unwrap_or(("disconnected".to_string(), "".to_string()));

        Self {
            status,
            network,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(2),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            if let Some((status, network)) = get_wifi_status() {
                self.status = status;
                self.network = network;
            }

            self.last_update = now;
        }
    }

    pub fn render(&self) -> String {
        let icon = if self.status == "connected" {
            "󰤨"
        } else {
            "󰤮"
        };

        let network_text = if self.status == "connected" && !self.network.is_empty() {
            &self.network
        } else {
            "Off"
        };

        format!("{} {}", icon, network_text)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.render());
        if colorize {
            let color = if self.status == "disconnected" {
                Color::Red // Disconnected: Red
            } else {
                Color::Blue // Connected: Blue
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }
}

fn get_wifi_status() -> Option<(String, String)> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "TYPE,STATE,CONNECTION", "device"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = str::from_utf8(&output.stdout).ok()?;

    for line in stdout.lines() {
        if line.starts_with("wifi:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let state = parts[1].to_lowercase();
                let connection = parts[2].to_string();

                if state == "connected" {
                    return Some(("connected".to_string(), connection));
                } else {
                    return Some(("disconnected".to_string(), "".to_string()));
                }
            }
        }
    }

    Some(("disconnected".to_string(), "".to_string()))
}
