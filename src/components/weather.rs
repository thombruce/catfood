use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone)]
pub struct WeatherData {
    pub temperature: String,
    pub condition: String,
    pub icon: String,
}

#[derive(Debug)]
pub struct Weather {
    data: Arc<Mutex<WeatherData>>,
    last_update: Arc<Mutex<u64>>,
    _update_handle: tokio::task::JoinHandle<()>,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<WeatherCondition>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherCondition {
    main: String,
}

impl Weather {
    pub fn new() -> Self {
        let data = Arc::new(Mutex::new(WeatherData {
            temperature: "--".to_string(),
            condition: "Unknown".to_string(),
            icon: "󰖐".to_string(),
        }));
        let last_update = Arc::new(Mutex::new(0u64));

        let data_clone = data.clone();
        let last_update_clone = last_update.clone();

        // Spawn background task for weather updates
        let update_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(600)); // 10 minutes

            loop {
                interval.tick().await;

                if let Ok(weather_data) = Self::fetch_weather_async().await {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if let Ok(mut data_guard) = data_clone.lock() {
                        data_guard.temperature = format!("{:.0}", weather_data.main.temp);
                        data_guard.condition = weather_data.weather[0].main.clone();
                        data_guard.icon = Self::get_weather_icon(&data_guard.condition);
                    }

                    if let Ok(mut last_update_guard) = last_update_clone.lock() {
                        *last_update_guard = now;
                    }
                }
            }
        });

        Self {
            data,
            last_update,
            _update_handle: update_handle,
        }
    }

    pub fn update(&mut self) {
        // This is now non-blocking - data is updated in background
        // Just check if we need to trigger initial update
        let _now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Ok(last_update_guard) = self.last_update.lock()
            && *last_update_guard == 0
        {
            // First run, spawn immediate fetch
            let data_clone = self.data.clone();
            let last_update_clone = self.last_update.clone();

            tokio::spawn(async move {
                if let Ok(weather_data) = Self::fetch_weather_async().await {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if let Ok(mut data_guard) = data_clone.lock() {
                        data_guard.temperature = format!("{:.0}", weather_data.main.temp);
                        data_guard.condition = weather_data.weather[0].main.clone();
                        data_guard.icon = Self::get_weather_icon(&data_guard.condition);
                    }

                    if let Ok(mut last_update_guard) = last_update_clone.lock() {
                        *last_update_guard = now;
                    }
                }
            });
        }
    }

    pub fn get_weather_data(&self) -> WeatherData {
        self.data
            .lock()
            .unwrap_or_else(|_| panic!("Weather data mutex poisoned"))
            .clone()
    }

    pub fn render(&self) -> String {
        let data = self.get_weather_data();
        format!("{} {}°C", data.icon, data.temperature)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(self.render());
        if colorize {
            let data = self.get_weather_data();
            let color = {
                let condition_lower = data.condition.to_lowercase();
                if condition_lower.contains("clear") || condition_lower.contains("sunny") {
                    Color::Yellow // Clear/Sunny: Yellow
                } else if condition_lower.contains("cloud") || condition_lower.contains("overcast")
                {
                    Color::Gray // Cloudy/Overcast: Gray
                } else if condition_lower.contains("rain") || condition_lower.contains("drizzle") {
                    Color::Blue // Rain/Drizzle: Blue
                } else if condition_lower.contains("snow") || condition_lower.contains("sleet") {
                    Color::Cyan // Snow/Sleet: Cyan
                } else if condition_lower.contains("thunder") || condition_lower.contains("storm") {
                    Color::Magenta // Thunder/Storm: Magenta
                } else if condition_lower.contains("fog") || condition_lower.contains("mist") {
                    Color::DarkGray // Fog/Mist: Dark Gray
                } else if condition_lower.contains("wind") {
                    Color::LightGreen // Wind: Light Green
                } else {
                    Color::White // Unknown: White
                }
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }

    async fn fetch_weather_async() -> color_eyre::Result<WeatherResponse> {
        // Using a free weather API that doesn't require API key
        // Note: This uses wttr.in for current weather
        let url = "http://wttr.in/?format=j1";

        let response = reqwest::get(url).await?;
        let json: serde_json::Value = response.json().await?;

        // Parse wttr.in response format
        if let Some(current) = json["current_condition"].get(0) {
            let temp = current["temp_C"]
                .as_str()
                .unwrap_or("--")
                .parse::<f64>()
                .unwrap_or(0.0);
            let condition = current["weatherDesc"][0]["value"]
                .as_str()
                .unwrap_or("Unknown");

            return Ok(WeatherResponse {
                main: Main { temp },
                weather: vec![WeatherCondition {
                    main: condition.to_string(),
                }],
            });
        }

        Err(color_eyre::eyre::eyre!("Failed to parse weather data"))
    }

    fn get_weather_icon(condition: &str) -> String {
        let condition_lower = condition.to_lowercase();
        match condition_lower.as_str() {
            cond if cond.contains("clear") || cond.contains("sunny") => "󰖙".to_string(),
            cond if cond.contains("cloud") || cond.contains("overcast") => "󰖐".to_string(),
            cond if cond.contains("rain") || cond.contains("drizzle") => "󰖗".to_string(),
            cond if cond.contains("snow") || cond.contains("sleet") => "󰖘".to_string(),
            cond if cond.contains("thunder") || cond.contains("storm") => "󰖓".to_string(),
            cond if cond.contains("fog") || cond.contains("mist") => "󰖑".to_string(),
            cond if cond.contains("wind") => "󰖝".to_string(),
            _ => "󰖐".to_string(),
        }
    }
}
