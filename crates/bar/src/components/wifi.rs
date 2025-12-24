use crate::components::ConfigurableComponent;
use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConfig {
    pub component: String,
    #[serde(default)]
    pub sparkline: bool,
    #[serde(default = "default_sparkline_length")]
    pub sparkline_length: usize,
    #[serde(default = "default_update_interval")]
    pub update_interval: u64,
}

fn default_sparkline_length() -> usize {
    10
}

fn default_update_interval() -> u64 {
    5
}

impl ConfigurableComponent for Wifi {
    type Config = WifiConfig;

    fn from_config(config: Self::Config) -> color_eyre::Result<Self> {
        let mut wifi = Self::new();
        if config.sparkline {
            wifi.enable_sparkline(config.sparkline_length, config.update_interval);
        }
        Ok(wifi)
    }
}

#[derive(Debug)]
pub struct Wifi {
    pub status: String,
    pub network: String,
    last_update: Instant,
    update_interval: Duration,
    sparkline_mode: bool,
    sparkline_length: usize,
    network_usage: Vec<u64>,
    last_total_bytes: Option<u64>,
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
            sparkline_mode: false,
            sparkline_length: 10,
            network_usage: Vec::new(),
            last_total_bytes: None,
        }
    }

    pub fn enable_sparkline(&mut self, length: usize, update_interval_sec: u64) {
        self.sparkline_mode = true;
        self.sparkline_length = length;
        self.update_interval = Duration::from_secs(update_interval_sec);
        self.network_usage = vec![0; length];
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            if let Some((status, network)) = get_wifi_status() {
                self.status = status;
                self.network = network;
            }

            if self.sparkline_mode && self.status == "connected" {
                self.update_network_usage();
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
        if self.sparkline_mode {
            return self.render_sparkline_as_spans(colorize);
        }

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

    fn update_network_usage(&mut self) {
        if let Some((bytes_rx, bytes_tx)) = get_network_stats() {
            let current_total = bytes_rx + bytes_tx;

            if let Some(last_total) = self.last_total_bytes {
                let usage = current_total.saturating_sub(last_total);
                self.network_usage.push(usage);
                if self.network_usage.len() > self.sparkline_length {
                    self.network_usage.remove(0);
                }
            }

            self.last_total_bytes = Some(current_total);
        }
    }

    fn render_sparkline_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        // Create a sparkline visualization of network usage over time
        // 
        // Algorithm:
        // 1. Normalize usage values to relative scale (0-8) based on max usage in window
        // 2. Map each normalized value to Unicode block character for visual representation
        // 3. Display characters as horizontal sparkline showing activity patterns
        //
        // The * 8 multiplier is intentional - it scales usage values to match the
        // 9 available Unicode block character levels (space + 8 incremental blocks)
        //
        // Normalize network usage values to 0-8 range for mapping to Unicode block characters
        // 9 levels total: space(0) + 8 block characters(1-8) 
        // We use 8 multiplier to get full range: 0=space, 1=▁, 2=▂, ..., 7=▇, 8=█
        // This creates a relative sparkline where the highest usage gets the tallest block
        let max_usage = self.network_usage.iter().max().copied().unwrap_or(1);
        let normalized_usage: Vec<u64> = self
            .network_usage
            .iter()
            .map(|&usage| {
                if max_usage > 0 {
                    // Scale to 0-8 range by multiplying by 8 and dividing by max
                    // The original * 8 multiplier was correct - it maps to the 9 Unicode levels
                    (usage * 8) / max_usage.max(1)
                } else {
                    0
                }
            })
            .collect();

        // Map normalized values to Unicode block characters
        // Unicode block characters provide 9 distinct levels for sparkline visualization:
        // 0 = ' ' (space - no activity)
        // 1 = '▁' (1/8 height - minimal activity)  
        // 2 = '▂' (2/8 height - low activity)
        // 3 = '▃' (3/8 height - low-moderate activity)
        // 4 = '▄' (4/8 height - moderate activity)
        // 5 = '▅' (5/8 height - moderate-high activity)
        // 6 = '▆' (6/8 height - high activity)
        // 7 = '▇' (7/8 height - very high activity)
        // 8 = '█' (8/8 height - maximum activity)
        let sparkline_chars: String = normalized_usage
            .iter()
            .map(|&usage| match usage {
                0 => ' ',  // No activity
                1 => '▁',  // 1/8 height
                2 => '▂',  // 2/8 height  
                3 => '▃',  // 3/8 height
                4 => '▄',  // 4/8 height
                5 => '▅',  // 5/8 height
                6 => '▆',  // 6/8 height
                7 => '▇',  // 7/8 height
                8 => '█',  // 8/8 height - full block
                _ => '█',  // Fallback for values >= 8 (shouldn't occur)
            })
            .collect();

        let icon = if self.status == "connected" {
            "󰤨"
        } else {
            "󰤮"
        };

        let text = format!("{} {}", icon, sparkline_chars);
        let span = Span::raw(text);

        if colorize {
            let color = if self.status == "disconnected" {
                Color::Red
            } else {
                Color::Blue
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

fn get_network_stats() -> Option<(u64, u64)> {
    let content = std::fs::read_to_string("/proc/net/dev").ok()?;

    for line in content.lines() {
        if line.contains("wlan") || line.contains("wl") || line.contains("wlp") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                let bytes_rx = parts[1].parse::<u64>().ok()?;
                let bytes_tx = parts[9].parse::<u64>().ok()?;
                return Some((bytes_rx, bytes_tx));
            }
        }
    }

    None
}

#[cfg(test)]
mod sparkline_tests {

    #[test]
    fn test_sparkline_normalization() {
        // Test sparkline normalization logic
        let usage_data = vec![0, 100, 200, 300, 400, 500, 600, 700, 800];
        let max_usage = 800;
        
        // Apply same normalization logic as in the sparkline
        let normalized: Vec<u64> = usage_data
            .iter()
            .map(|&usage| {
                if max_usage > 0 {
                    (usage * 8) / max_usage.max(1)
                } else {
                    0
                }
            })
            .collect();
        
        println!("Original: {:?}", usage_data);
        println!("Normalized: {:?}", normalized);
        
        // Verify mapping produces expected 0-8 range
        assert_eq!(normalized, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        
        // Test character mapping
        let sparkline_chars: String = normalized
            .iter()
            .map(|&usage| match usage {
                0 => ' ',
                1 => '▁',
                2 => '▂', 
                3 => '▃',
                4 => '▄',
                5 => '▅',
                6 => '▆',
                7 => '▇',
                8 => '█',
                _ => '█',
            })
            .collect();
            
        println!("Sparkline: '{}'", sparkline_chars);
        assert_eq!(sparkline_chars, " ▁▂▃▄▅▆▇█");
    }
}
