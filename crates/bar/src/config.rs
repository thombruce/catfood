use crate::time_utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComponentConfig {
    String(String),
    Object(ComponentOptions),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOptions {
    pub name: String,
    #[serde(default)]
    pub sparkline: Option<bool>,
    #[serde(default)]
    pub sparkline_length: Option<usize>,
    #[serde(default)]
    pub sparkline_update_freq: Option<u64>,
    #[serde(default)]
    pub sparkline_logarithmic: Option<bool>,
    #[serde(default)]
    pub socket_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bars: BarsConfig,
    pub colorize: bool,
    #[serde(default = "time_utils::default_day_start")]
    pub day_start: u8,
    #[serde(default = "time_utils::default_night_start")]
    pub night_start: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarsConfig {
    pub left: Vec<ComponentConfig>,
    pub middle: Vec<ComponentConfig>,
    pub right: Vec<ComponentConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bars: BarsConfig {
                left: vec![
                    ComponentConfig::String("workspaces".to_string()),
                    ComponentConfig::String("windows".to_string()),
                ],
                middle: vec![
                    ComponentConfig::String("time".to_string()),
                    ComponentConfig::String("separator".to_string()),
                    ComponentConfig::String("weather".to_string()),
                ],
                right: vec![
                    ComponentConfig::String("temperature".to_string()),
                    ComponentConfig::String("space".to_string()),
                    ComponentConfig::String("cpu".to_string()),
                    ComponentConfig::String("space".to_string()),
                    ComponentConfig::String("ram".to_string()),
                    ComponentConfig::String("separator".to_string()),
                    ComponentConfig::String("wifi".to_string()),
                    ComponentConfig::String("separator".to_string()),
                    ComponentConfig::String("brightness".to_string()),
                    ComponentConfig::String("space".to_string()),
                    ComponentConfig::String("volume".to_string()),
                    ComponentConfig::String("separator".to_string()),
                    ComponentConfig::String("battery".to_string()),
                ],
            },
            colorize: true,
            day_start: time_utils::default_day_start(),
            night_start: time_utils::default_night_start(),
        }
    }
}

impl Config {
    pub fn load() -> color_eyre::Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = Config::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    pub fn save(&self) -> color_eyre::Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> std::path::PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home_dir)
            .join(".config")
            .join("catfood")
            .join("bar.json")
    }

    pub fn get_components_for_bar(&self, bar: &str) -> Option<&Vec<ComponentConfig>> {
        match bar {
            "left" => Some(&self.bars.left),
            "middle" => Some(&self.bars.middle),
            "right" => Some(&self.bars.right),
            _ => None,
        }
    }

    pub fn reload(&self) -> color_eyre::Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}

impl ComponentConfig {
    pub fn name(&self) -> &str {
        match self {
            ComponentConfig::String(name) => name,
            ComponentConfig::Object(options) => &options.name,
        }
    }

    pub fn sparkline(&self) -> Option<bool> {
        match self {
            ComponentConfig::String(_) => None,
            ComponentConfig::Object(options) => options.sparkline,
        }
    }

    pub fn sparkline_length(&self) -> Option<usize> {
        match self {
            ComponentConfig::String(_) => None,
            ComponentConfig::Object(options) => options.sparkline_length,
        }
    }

    pub fn sparkline_update_freq(&self) -> Option<u64> {
        match self {
            ComponentConfig::String(_) => None,
            ComponentConfig::Object(options) => options.sparkline_update_freq,
        }
    }

    pub fn sparkline_logarithmic(&self) -> Option<bool> {
        match self {
            ComponentConfig::String(_) => None,
            ComponentConfig::Object(options) => options.sparkline_logarithmic,
        }
    }

    pub fn socket_path(&self) -> Option<String> {
        match self {
            ComponentConfig::String(_) => None,
            ComponentConfig::Object(options) => options.socket_path.clone(),
        }
    }
}
