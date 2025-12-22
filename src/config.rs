use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bars: BarsConfig,
    pub colorize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarsConfig {
    pub left: Vec<String>,
    pub middle: Vec<String>,
    pub right: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bars: BarsConfig {
                left: vec!["workspaces".to_string(), "windows".to_string()],
                middle: vec![
                    "time".to_string(),
                    "separator".to_string(),
                    "weather".to_string(),
                ],
                right: vec![
                    "temperature".to_string(),
                    "space".to_string(),
                    "cpu".to_string(),
                    "space".to_string(),
                    "ram".to_string(),
                    "separator".to_string(),
                    "wifi".to_string(),
                    "separator".to_string(),
                    "brightness".to_string(),
                    "space".to_string(),
                    "volume".to_string(),
                    "separator".to_string(),
                    "battery".to_string(),
                ],
            },
            colorize: true,
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
            .join("catfood_bar")
            .join("config.json")
    }

    pub fn get_components_for_bar(&self, bar: &str) -> Option<&Vec<String>> {
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
