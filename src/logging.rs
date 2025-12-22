use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
}

#[derive(Debug)]
struct Logger {
    file: std::fs::File,
}

impl Logger {
    fn new() -> color_eyre::Result<Self> {
        // Get XDG data directory, fallback to ~/.local/share
        let data_dir = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/.local/share", home)
        });

        // Create logs directory
        let log_dir = PathBuf::from(data_dir).join("catfood_bar").join("logs");
        std::fs::create_dir_all(&log_dir)?;

        // Create log file
        let log_path = log_dir.join("catfood_bar.log");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        // Initialize log with header
        let timestamp = Utc::now().to_rfc3339();
        writeln!(file, "\n\n=== catfood_bar started at {} ===", timestamp)?;

        Ok(Self { file })
    }

    fn rotate_log_if_needed(&mut self) -> color_eyre::Result<()> {
        use std::io::{Read, Seek, SeekFrom};

        // Check current file size and lines
        let current_size = self.file.metadata()?.len();

        // If file is getting large, rotate to last 1000 lines
        if current_size > 1024 * 1024 {
            // 1MB threshold
            // Read all lines and keep last 1000
            let current_pos = self.file.stream_position()?;
            self.file.seek(SeekFrom::Start(0))?;

            let mut content = String::new();
            self.file.read_to_string(&mut content)?;

            let lines: Vec<&str> = content.lines().collect();
            if lines.len() > 1000 {
                let keep_lines = &lines[lines.len() - 1000..];
                let new_content = keep_lines.join("\n");

                // Truncate and write new content
                self.file.set_len(0)?;
                self.file.seek(SeekFrom::Start(0))?;
                self.file.write_all(new_content.as_bytes())?;
            } else {
                self.file.seek(SeekFrom::Start(current_pos))?;
            }
        }

        Ok(())
    }

    fn write_log(&mut self, category: &str, message: &str) -> color_eyre::Result<()> {
        self.rotate_log_if_needed()?;

        let timestamp = Utc::now().to_rfc3339();
        writeln!(
            self.file,
            "{} [ERROR] [{}] {}",
            timestamp, category, message
        )?;
        self.file.flush()?;
        Ok(())
    }
}

pub fn log_file_watcher_error(error: &str) {
    if let Ok(mut logger) = LOGGER.lock() {
        if logger.is_none()
            && let Ok(new_logger) = Logger::new()
        {
            *logger = Some(new_logger);
        }

        if let Some(ref mut log) = *logger {
            let _ = log.write_log("FILE_WATCHER", error);
        }
    }
}

pub fn log_config_error(error: &str) {
    if let Ok(mut logger) = LOGGER.lock() {
        if logger.is_none()
            && let Ok(new_logger) = Logger::new()
        {
            *logger = Some(new_logger);
        }

        if let Some(ref mut log) = *logger {
            let _ = log.write_log("CONFIG", error);
        }
    }
}

pub fn log_component_error(component_name: &str, error: &str) {
    if let Ok(mut logger) = LOGGER.lock() {
        if logger.is_none()
            && let Ok(new_logger) = Logger::new()
        {
            *logger = Some(new_logger);
        }

        if let Some(ref mut log) = *logger {
            let _ = log.write_log(
                &format!("COMPONENT_{}", component_name.to_uppercase()),
                error,
            );
        }
    }
}

pub fn log_system_error(_context: &str, error: &str) {
    if let Ok(mut logger) = LOGGER.lock() {
        if logger.is_none()
            && let Ok(new_logger) = Logger::new()
        {
            *logger = Some(new_logger);
        }

        if let Some(ref mut log) = *logger {
            let _ = log.write_log("SYSTEM", error);
        }
    }
}
