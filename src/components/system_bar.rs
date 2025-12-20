use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use regex::Regex;
use std::process::Command;
use sysinfo::{Components, CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

static BRIGHTNESS_REGEX: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\d+%").unwrap());

#[derive(Debug)]
pub struct SystemBar {
    temperature: String,
    cpu_usage: String,
    ram_usage: String,
    wifi_status: String,
    wifi_network: String,
    brightness: String,
    volume: String,
    battery_percentage: String,
    system: System,
    components: Components,
    battery_manager: battery::Manager,
    battery: battery::Battery,
}

impl SystemBar {
    pub fn new() -> color_eyre::Result<Self> {
        let system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        let components = Components::new();

        let manager = battery::Manager::new()?;
        let battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e.into());
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
            }
        };

        let (wifi_status, wifi_network) =
            get_wifi_status().unwrap_or(("disconnected".to_string(), "".to_string()));

        Ok(Self {
            temperature: "0".to_string(),
            cpu_usage: "0".to_string(),
            ram_usage: "0".to_string(),
            wifi_status,
            wifi_network,
            brightness: get_system_brightness().unwrap_or_default(),
            volume: get_system_volume().unwrap_or(0).to_string(),
            battery_percentage: ((battery.state_of_charge().value * 100.0) as i32).to_string(),
            system,
            components,
            battery_manager: manager,
            battery,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        // Update CPU and temperature
        self.system.refresh_cpu_all();
        self.components.refresh(true);

        // Update temperature
        if let Some(component) = self.components.iter().find(|c| {
            c.label().to_lowercase().contains("cpu")
                || c.label().to_lowercase().contains("core")
                || c.label().to_lowercase().contains("package")
        }) && let Some(temp) = component.temperature()
        {
            self.temperature = format!("{:.0}", temp);
        }

        let iter = self.system.cpus().iter();
        let count = iter.len() as f32;
        let sum = iter.fold(0.0, |acc, x| acc + x.cpu_usage());
        let avg: u32 = (sum / count) as u32;
        self.cpu_usage = avg.to_string();

        // Update RAM
        self.system.refresh_memory();
        let mem_percent: u32 =
            (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as u32;
        self.ram_usage = mem_percent.to_string();

        // Update brightness and volume
        self.brightness = get_system_brightness().unwrap_or_default();
        self.volume = get_system_volume().unwrap_or(0).to_string();

        // Update battery
        self.battery_manager.refresh(&mut self.battery)?;
        self.battery_percentage =
            ((self.battery.state_of_charge().value * 100.0) as i32).to_string();

        // Update WiFi
        if let Some((status, network)) = get_wifi_status() {
            self.wifi_status = status;
            self.wifi_network = network;
        }

        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let wifi_icon = if self.wifi_status == "connected" {
            "󰤨 "
        } else {
            "󰤮 "
        };

        let wifi_text = if self.wifi_status == "connected" && !self.wifi_network.is_empty() {
            &self.wifi_network
        } else {
            "Off"
        };

        let spans = vec![
            Span::raw(" "),
            Span::raw(self.temperature.clone() + "°C"),
            Span::raw(" "),
            Span::raw("󰻠 "),
            Span::raw(self.cpu_usage.clone() + "%"),
            Span::raw(" "),
            Span::raw("󰍛 "),
            Span::raw(self.ram_usage.clone() + "%"),
            Span::raw(" | "),
            Span::raw(wifi_icon),
            Span::raw(wifi_text.to_string()),
            Span::raw(" | "),
            Span::raw("󰃠 "),
            Span::raw(self.brightness.clone()),
            Span::raw(" "),
            Span::raw("󰕾 "),
            Span::raw(self.volume.clone() + "%"),
            Span::raw(" | "),
            Span::raw("󰁹 "),
            Span::raw(self.battery_percentage.clone() + "%"),
        ];

        let system_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(system_line).right_aligned().fg(Color::White),
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
        let parts: Vec<&str> = stdout.split_whitespace().collect();

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
