use ratatui::{prelude::Stylize, style::Color, text::Span};
use std::collections::HashMap;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Vpn {
    pub status: String,
    pub country: String,
    pub short: String,
    last_update: Instant,
    update_interval: Duration,
}

impl Vpn {
    pub fn new() -> Self {
        let (status, country, short) = get_vpn_status().unwrap_or((
            "disconnected".to_string(),
            "".to_string(),
            "--".to_string(),
        ));

        Self {
            status,
            country,
            short,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(5),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            if let Some((status, country, short)) = get_vpn_status() {
                self.status = status;
                self.country = country;
                self.short = short;
            }

            self.last_update = now;
        }
    }

    pub fn render(&self) -> String {
        format!(" {}", self.short)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.render());
        if colorize {
            let color = if self.status == "disconnected" {
                Color::DarkGray // Disconnected: Dark Gray
            } else {
                Color::Magenta // Connected: Purple
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }
}

fn get_vpn_status() -> Option<(String, String, String)> {
    let output = Command::new("nordvpn").args(["status"]).output().ok()?;

    if !output.status.success() {
        return Some(("disconnected".to_string(), "".to_string(), "--".to_string()));
    }

    let stdout = str::from_utf8(&output.stdout).ok()?;
    let mut status = "disconnected".to_string();
    let mut country = "".to_string();

    for line in stdout.lines() {
        if line.contains("Status:") {
            if line.contains("Connected") {
                status = "connected".to_string();
            }
        } else if line.contains("Country:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                country = parts[1].trim().to_string();
            }
        }
    }

    let short = if status == "connected" && !country.is_empty() {
        get_country_short_name(&country)
    } else {
        "--".to_string() // VPN disconnected indicator
    };

    Some((status, country, short))
}

fn get_country_short_name(country: &str) -> String {
    let country_codes = HashMap::from([
        ("United States", "US"),
        ("United Kingdom", "GB"),
        ("Germany", "DE"),
        ("France", "FR"),
        ("Netherlands", "NL"),
        ("Sweden", "SE"),
        ("Norway", "NO"),
        ("Denmark", "DK"),
        ("Finland", "FI"),
        ("Switzerland", "CH"),
        ("Austria", "AT"),
        ("Belgium", "BE"),
        ("Italy", "IT"),
        ("Spain", "ES"),
        ("Poland", "PL"),
        ("Czech Republic", "CZ"),
        ("Hungary", "HU"),
        ("Romania", "RO"),
        ("Bulgaria", "BG"),
        ("Croatia", "HR"),
        ("Slovakia", "SK"),
        ("Slovenia", "SI"),
        ("Estonia", "EE"),
        ("Latvia", "LV"),
        ("Lithuania", "LT"),
        ("Portugal", "PT"),
        ("Greece", "GR"),
        ("Ireland", "IE"),
        ("Luxembourg", "LU"),
        ("Cyprus", "CY"),
        ("Malta", "MT"),
        ("Canada", "CA"),
        ("Australia", "AU"),
        ("New Zealand", "NZ"),
        ("Japan", "JP"),
        ("South Korea", "KR"),
        ("Singapore", "SG"),
        ("India", "IN"),
        ("Brazil", "BR"),
        ("Argentina", "AR"),
        ("Chile", "CL"),
        ("Mexico", "MX"),
        ("South Africa", "ZA"),
        ("Turkey", "TR"),
        ("Israel", "IL"),
        ("UAE", "AE"),
        ("Saudi Arabia", "SA"),
        ("Egypt", "EG"),
        ("Morocco", "MA"),
        ("Nigeria", "NG"),
        ("Kenya", "KE"),
        ("Thailand", "TH"),
        ("Vietnam", "VN"),
        ("Philippines", "PH"),
        ("Indonesia", "ID"),
        ("Malaysia", "MY"),
        ("Hong Kong", "HK"),
        ("Taiwan", "TW"),
    ]);

    country_codes
        .get(country)
        .copied()
        .unwrap_or("--")
        .to_string()
}
