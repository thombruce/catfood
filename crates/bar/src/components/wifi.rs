use super::sparkline::Sparkline;
use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Wifi {
    pub status: String,
    pub network: String,
    cached_span_content: String,
    last_update: Instant,
    update_interval: Duration,
    sparkline: Sparkline,
    last_bytes: Option<u64>,
}

impl Wifi {
    pub fn with_config(
        sparkline: bool,
        sparkline_length: usize,
        sparkline_update_freq: u64,
        sparkline_logarithmic: bool,
    ) -> Self {
        let (status, network) =
            get_wifi_status().unwrap_or(("disconnected".to_string(), "".to_string()));

        let icon = if status == "connected" {
            "󰤨"
        } else {
            "󰤮"
        };

        let network_text = if status == "connected" && !network.is_empty() {
            &network
        } else {
            "Off"
        };

        let sparkline = Sparkline::new(sparkline, sparkline_length, sparkline_logarithmic);
        let cached_span_content = if sparkline.enabled {
            format!("{} {}", icon, sparkline.render_with_spaces())
        } else {
            format!("{} {}", icon, network_text)
        };

        Self {
            status,
            network,
            cached_span_content,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(sparkline_update_freq),
            sparkline,
            last_bytes: None,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            if let Some((status, network)) = get_wifi_status() {
                self.status = status;
                self.network = network;

                let icon = if self.status == "connected" {
                    "󰤨"
                } else {
                    "󰤮"
                };

                if self.sparkline.enabled {
                    if let Some(current_bytes) = get_network_usage() {
                        let usage = if let Some(last_bytes) = self.last_bytes {
                            current_bytes.saturating_sub(last_bytes)
                        } else {
                            0
                        };

                        self.last_bytes = Some(current_bytes);

                        // Update sparkline data
                        self.sparkline.update(usage);

                        // Render sparkline
                        self.cached_span_content = format!("{} {}", icon, self.sparkline.render());
                    } else {
                        self.cached_span_content =
                            format!("{} {}", icon, self.sparkline.render_with_spaces());
                    }
                } else {
                    let network_text = if self.status == "connected" && !self.network.is_empty() {
                        &self.network
                    } else {
                        "Off"
                    };
                    self.cached_span_content = format!("{} {}", icon, network_text);
                }
            }

            self.last_update = now;
        }
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(&self.cached_span_content);
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

fn get_network_usage() -> Option<u64> {
    let content = std::fs::read_to_string("/proc/net/dev").ok()?;

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            let interface = parts[0].trim_end_matches(':');
            // Look for wireless interfaces (common prefixes)
            if interface.starts_with("wlan")
                || interface.starts_with("wifi")
                || interface.starts_with("wl")
            {
                // Return sum of received and transmitted bytes
                let rx_bytes: u64 = parts[1].parse().ok()?;
                let tx_bytes: u64 = parts[9].parse().ok()?;
                return Some(rx_bytes + tx_bytes);
            }
        }
    }

    None
}
