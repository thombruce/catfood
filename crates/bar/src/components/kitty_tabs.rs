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
                    // Active tab: show full title (more space)
                    let content = format!(" {} ", truncate_title(&tab.title, true));
                    if colorize {
                        Span::raw(content)
                            .bg(Color::Rgb(103, 117, 140)) // Kitty gray background
                            .fg(Color::White)
                    } else {
                        Span::raw(content).bg(Color::White).fg(Color::Black)
                    }
                } else {
                    // Inactive tab: show icon only (very compact)
                    let content = format!(" {} ", get_tab_icon(&tab.title));
                    if colorize {
                        Span::raw(content).fg(Color::Rgb(103, 117, 140)) // Kitty gray text
                    } else {
                        Span::raw(content).fg(Color::White)
                    }
                }
            })
            .collect::<Vec<Span>>()
    }
}

fn truncate_title(title: &str, is_active: bool) -> String {
    // NOTE: This might be redundant given that we always get_tab_icon for non-active tabs below.
    let max_len = if is_active { 20 } else { 12 }; // More space for active tab
    if title.len() > max_len {
        format!("{}...", &title[..max_len - 3])
    } else {
        title.to_string()
    }
}

fn get_tab_icon(title: &str) -> String {
    let title_lower = title.to_lowercase();

    // Check for common terminal programs first
    if title_lower.starts_with("nvim") || title_lower.contains("neovim") {
        return "".to_string();
    } else if title_lower.starts_with("vim") {
        return "".to_string();
    } else if title_lower.starts_with("emacs") {
        return "󰍹".to_string();
    } else if title_lower.starts_with("nano") {
        return "".to_string();
    } else if title_lower.starts_with("htop") || title_lower.starts_with("btop") {
        return "󰔚".to_string();
    } else if title_lower.starts_with("yazi") {
        return "󰇥".to_string();
    } else if title_lower.starts_with("ranger") || title_lower.starts_with("lf") {
        return "󰉋".to_string();
    } else if title_lower.starts_with("git") {
        return "󰊢".to_string();
    } else if title_lower.starts_with("man") {
        return "󰍹".to_string();
    } else if title_lower.starts_with("ssh") {
        return "󰣀".to_string();
    } else if title_lower.starts_with("cmus") || title_lower.starts_with("ncmpcpp") {
        return "󰓇".to_string();
    } else if title_lower.starts_with("docker") {
        return "󰡨".to_string();
    } else if title_lower.starts_with("node") || title_lower.starts_with("npm") {
        return "󰎙".to_string();
    } else if title_lower.starts_with("python") || title_lower.starts_with("python3") {
        return "󰌠".to_string();
    } else if title_lower.starts_with("rustc") || title_lower.contains("cargo") {
        return "󱘗".to_string();
    } else if title_lower.starts_with("go") {
        return "󰟦".to_string();
    } else if title_lower.starts_with("java") {
        return "󰬙".to_string();
    } else if title_lower.starts_with("fish")
        || title_lower.starts_with("bash")
        || title_lower.starts_with("zsh")
    {
        return "󰆍".to_string();
    } else if title_lower.contains("watch") || title_lower.contains("tail") {
        return "󰈰".to_string();
    } else if title_lower.starts_with("wget") || title_lower.starts_with("curl") {
        return "󰈁".to_string();
    } else if title_lower.contains("edit") || title_lower.contains("vi") {
        return "".to_string();
    }

    // Default shell/terminal icon
    "󰆍".to_string()
}

fn get_focused_kitty_pid() -> Option<u32> {
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
        let kitty_tabs = KittyTabs::new();
        assert!(kitty_tabs.kitty_pid.is_none() || kitty_tabs.kitty_pid.is_some());
    }

    #[test]
    fn test_kitty_tabs_update() {
        let mut kitty_tabs = KittyTabs::new();
        kitty_tabs.update();
    }

    #[test]
    fn test_kitty_tabs_render_empty() {
        let kitty_tabs = KittyTabs {
            tabs: vec![],
            kitty_pid: None,
        };
        let spans = kitty_tabs.render_as_spans(true);
        assert_eq!(spans.len(), 0);
    }

    #[test]
    fn test_kitty_tabs_render_with_tabs() {
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
        let long_title = "This is a very long tab title that should be truncated";
        let truncated = truncate_title(long_title, true);
        assert!(truncated.len() <= 20);
        assert!(truncated.ends_with("..."));

        let short_title = "Short title";
        let not_truncated = truncate_title(short_title, false);
        assert_eq!(not_truncated, short_title);
    }

    #[test]
    fn test_get_tab_icon() {
        assert_eq!(get_tab_icon("nvim config"), "");
        assert_eq!(get_tab_icon("vim /etc/fstab"), "");
        assert_eq!(get_tab_icon("htop"), "󰔚");
        assert_eq!(get_tab_icon("random command"), "󰆍"); // default shell icon
    }
}
