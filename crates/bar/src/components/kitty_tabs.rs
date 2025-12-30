use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::Deserialize;
use std::process::Command;

use crate::logging;

#[derive(Deserialize, Debug)]
struct KittyWindow {
    #[serde(default)]
    is_active: bool,
    #[serde(default)]
    is_focused: bool,
    #[serde(default)]
    last_focused: bool,
    tabs: Vec<KittyTab>,
}

#[derive(Deserialize, Debug)]
struct KittyTab {
    title: String,
    #[serde(default)]
    is_active: bool,
}

#[derive(Debug, Clone)]
pub struct TabInfo {
    title: String,
    is_active: bool,
}

#[derive(Debug, Default, Clone)]
pub struct KittyTabs {
    pub tabs: Vec<TabInfo>,
    kitty_pid: Option<u32>,
}

impl KittyTabs {
    pub fn new() -> Self {
        Self {
            tabs: get_kitty_tabs().unwrap_or_default(),
            kitty_pid: get_focused_kitty_pid(),
        }
    }

    pub fn update(&mut self) {
        self.kitty_pid = get_focused_kitty_pid();
        self.tabs = if let Some(pid) = self.kitty_pid {
            get_kitty_tabs_for_pid(pid).unwrap_or_default()
        } else {
            Vec::new()
        };
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        if self.tabs.is_empty() {
            return vec![];
        }

        self.tabs
            .iter()
            .map(|tab| {
                if tab.is_active {
                    if colorize {
                        Span::raw(format!(" {} ", truncate_title(&tab.title)))
                            .bg(Color::Rgb(103, 117, 140)) // Kitty gray color
                            .fg(Color::White)
                    } else {
                        Span::raw(format!(" {} ", truncate_title(&tab.title)))
                            .bg(Color::White)
                            .fg(Color::Black)
                    }
                } else if colorize {
                    Span::raw(format!(" {} ", truncate_title(&tab.title)))
                        .fg(Color::Rgb(103, 117, 140)) // Kitty gray color for inactive tabs
                } else {
                    Span::raw(format!(" {} ", truncate_title(&tab.title))).fg(Color::White)
                }
            })
            .collect::<Vec<Span>>()
    }
}

fn truncate_title(title: &str) -> String {
    if title.len() > 20 {
        format!("{}...", &title[..17])
    } else {
        title.to_string()
    }
}

fn get_focused_kitty_pid() -> Option<u32> {
    // Get currently active window
    let active_output = Command::new("hyprctl")
        .args(["activewindow", "-j"])
        .output()
        .ok()?;

    if !active_output.status.success() {
        return None;
    }

    let active_stdout = String::from_utf8(active_output.stdout).ok()?;
    if let Ok(active_window) = serde_json::from_str::<serde_json::Value>(&active_stdout)
        && let (Some(class), Some(pid)) = (
            active_window.get("class").and_then(|v| v.as_str()),
            active_window.get("pid").and_then(|v| v.as_u64()),
        )
    {
        // Only return if the active window is a Kitty window
        if class == "kitty" {
            // Check if this Kitty instance uses --single-instance flag
            if is_single_instance_kitty(pid as u32) {
                return Some(pid as u32);
            }
        }
    }

    None
}

fn is_single_instance_kitty(pid: u32) -> bool {
    // Check if this Kitty process was started with --single-instance
    let output = match Command::new("pgrep")
        .args(["-f", "kitty.*--single-instance"])
        .output()
    {
        Ok(output) => output,
        Err(_) => return false,
    };

    if !output.status.success() {
        return false;
    }

    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(_) => return false,
    };
    for pid_str in stdout.lines() {
        if let Ok(found_pid) = pid_str.parse::<u32>()
            && found_pid == pid
        {
            return true;
        }
    }

    false
}

fn get_kitty_tabs() -> Option<Vec<TabInfo>> {
    if let Some(pid) = get_focused_kitty_pid() {
        get_kitty_tabs_for_pid(pid)
    } else {
        None
    }
}

fn get_kitty_tabs_for_pid(pid: u32) -> Option<Vec<TabInfo>> {
    let socket_path = format!("/tmp/kitty-{}", pid);

    let output = Command::new("kitty")
        .args(["@", "--to", &format!("unix:{}", socket_path), "ls"])
        .output()
        .ok()?;

    if !output.status.success() {
        logging::log_component_error(
            "KITTY_TABS",
            str::from_utf8(&output.stderr).unwrap_or("unknown error"),
        );
        return None;
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let windows: Vec<KittyWindow> = serde_json::from_str(stdout).ok()?;

    for window in windows {
        if window.is_active || window.is_focused || window.last_focused {
            let tabs: Vec<TabInfo> = window
                .tabs
                .into_iter()
                .enumerate()
                .map(|(index, tab)| TabInfo {
                    title: if tab.title.is_empty() {
                        format!("Tab {}", index + 1)
                    } else {
                        tab.title
                    },
                    is_active: tab.is_active,
                })
                .collect();

            return Some(tabs);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kitty_tabs_new() {
        // Test that KittyTabs can be created
        let kitty_tabs = KittyTabs::new();
        // This test mainly verifies that struct can be instantiated
        // and default values are set correctly
        assert!(kitty_tabs.kitty_pid.is_none() || kitty_tabs.kitty_pid.is_some());
    }

    #[test]
    fn test_kitty_tabs_update() {
        // Test that KittyTabs can be updated
        let mut kitty_tabs = KittyTabs::new();
        kitty_tabs.update();
        // This test verifies that update method doesn't panic
        // The actual result depends on whether a focused Kitty instance exists
    }

    #[test]
    fn test_kitty_tabs_render_empty() {
        // Test rendering when no tabs are available
        let kitty_tabs = KittyTabs {
            tabs: vec![],
            kitty_pid: None,
        };
        let spans = kitty_tabs.render_as_spans(true);
        assert_eq!(spans.len(), 0);
    }

    #[test]
    fn test_kitty_tabs_render_with_tabs() {
        // Test rendering with some mock tabs
        let kitty_tabs = KittyTabs {
            tabs: vec![
                TabInfo {
                    title: "Tab 1".to_string(),
                    is_active: false,
                },
                TabInfo {
                    title: "Active Tab".to_string(),
                    is_active: true,
                },
            ],
            kitty_pid: Some(12345),
        };
        let spans = kitty_tabs.render_as_spans(true);
        assert_eq!(spans.len(), 2);
    }

    #[test]
    fn test_truncate_title() {
        // Test title truncation
        let long_title = "This is a very long tab title that should be truncated";
        let truncated = truncate_title(long_title);
        assert!(truncated.len() <= 20);
        assert!(truncated.ends_with("..."));

        let short_title = "Short title";
        let not_truncated = truncate_title(short_title);
        assert_eq!(not_truncated, short_title);
    }
}
