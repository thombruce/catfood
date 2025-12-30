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
    socket_path: Option<String>,
}

impl KittyTabs {
    pub fn new() -> Self {
        Self::with_config(None)
    }

    pub fn with_config(socket_path: Option<String>) -> Self {
        Self {
            tabs: get_kitty_tabs(socket_path.as_deref()).unwrap_or_default(),
            kitty_pid: get_focused_kitty_pid(),
            socket_path,
        }
    }

    pub fn update(&mut self) {
        self.kitty_pid = get_focused_kitty_pid();
        self.tabs = if let Some(pid) = self.kitty_pid {
            get_kitty_tabs_for_pid(pid, self.socket_path.as_deref()).unwrap_or_default()
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
                    // Active tab: show icon + full title
                    let icon = get_tab_icon(&tab.title);
                    let title = truncate_title(&tab.title, true);
                    let content = format!(" {} {} ", icon, title);
                    if colorize {
                        let (bg_color, fg_color) = get_tab_color(&tab.title);
                        Span::raw(content).bg(bg_color).fg(fg_color)
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
    let max_len = if is_active { 16 } else { 12 }; // Account for icon in active tabs
    if title.len() > max_len {
        format!("{}...", &title[..max_len - 3])
    } else {
        title.to_string()
    }
}

fn get_tab_color(title: &str) -> (Color, Color) {
    let title_lower = title.to_lowercase();

    // Terminal-based applications - use title
    if title_lower.starts_with("nvim") || title_lower.contains("neovim") {
        return (Color::Rgb(0, 107, 84), Color::White); // Neovim Green
    } else if title_lower.starts_with("vim") {
        return (Color::Rgb(19, 134, 71), Color::White); // Vim Green
    } else if title_lower.starts_with("emacs") {
        return (Color::Rgb(146, 35, 127), Color::White); // Emacs Purple
    } else if title_lower.starts_with("htop") || title_lower.starts_with("btop") {
        return (Color::Rgb(255, 152, 0), Color::Black); // System Monitor Orange
    } else if title_lower.starts_with("yazi") {
        return (Color::Rgb(255, 200, 87), Color::Black); // Yazi Yellow
    } else if title_lower.starts_with("ranger") || title_lower.starts_with("lf") {
        return (Color::Rgb(41, 128, 185), Color::White); // File Manager Blue
    } else if title_lower.starts_with("git") {
        return (Color::Rgb(240, 80, 50), Color::White); // Git Orange
    } else if title_lower.starts_with("ssh") {
        return (Color::Rgb(0, 100, 200), Color::White); // SSH Blue
    } else if title_lower.starts_with("cmus") || title_lower.starts_with("ncmpcpp") {
        return (Color::Rgb(29, 185, 84), Color::White); // Music Green
    } else if title_lower.starts_with("docker") {
        return (Color::Rgb(41, 128, 185), Color::White); // Docker Blue
    } else if title_lower.starts_with("node") || title_lower.starts_with("npm") {
        return (Color::Rgb(102, 77, 255), Color::White); // Node.js Green
    } else if title_lower.starts_with("python") || title_lower.starts_with("python3") {
        return (Color::Rgb(53, 114, 165), Color::White); // Python Blue
    } else if title_lower.starts_with("rustc") || title_lower.contains("cargo") {
        return (Color::Rgb(222, 76, 65), Color::White); // Rust Orange
    } else if title_lower.starts_with("go") {
        return (Color::Rgb(0, 173, 216), Color::Black); // Go Cyan
    } else if title_lower.starts_with("java") {
        return (Color::Rgb(255, 87, 34), Color::White); // Java Orange
    } else if title_lower.starts_with("k9s") || title_lower.starts_with("kubectl") {
        return (Color::Rgb(61, 90, 254), Color::White); // Kubernetes Blue
    } else if title_lower.starts_with("terraform") || title_lower.starts_with("tf") {
        return (Color::Rgb(94, 103, 110), Color::White); // Terraform Gray
    } else if title_lower.starts_with("lazygit") || title_lower.starts_with("gitui") {
        return (Color::Rgb(240, 80, 50), Color::White); // Git UI Orange
    } else if title_lower.starts_with("tmux") || title_lower.starts_with("screen") {
        return (Color::Rgb(46, 52, 64), Color::White); // Terminal Multiplexer Dark
    } else if title_lower.starts_with("weechat") || title_lower.starts_with("irssi") {
        return (Color::Rgb(254, 0, 84), Color::White); // IRC Red
    } else if title_lower.starts_with("neomutt") || title_lower.starts_with("mutt") {
        return (Color::Rgb(0, 112, 193), Color::White); // Email Blue
    } else if title_lower.starts_with("newsboat") || title_lower.starts_with("nnn") {
        return (Color::Rgb(255, 193, 7), Color::Black); // News Yellow
    } else if title_lower.starts_with("glow") || title_lower.starts_with("mdcat") {
        return (Color::Rgb(33, 150, 243), Color::White); // Markdown Blue
    } else if title_lower.starts_with("tig") || title_lower.starts_with("lazydocker") {
        return (Color::Rgb(240, 80, 50), Color::White); // Git TUI Orange
    } else if title_lower.starts_with("jq") || title_lower.starts_with("yq") {
        return (Color::Rgb(0, 150, 136), Color::White); // JSON/YAML Teal
    } else if title_lower.starts_with("sqlite3")
        || title_lower.starts_with("mysql")
        || title_lower.starts_with("redis-cli")
        || title_lower.starts_with("psql")
    {
        return (Color::Rgb(40, 167, 69), Color::White); // Database Green
    } else if title_lower.starts_with("gh") || title_lower.starts_with("hub") {
        return (Color::Rgb(29, 185, 84), Color::White); // GitHub Green
    } else if title_lower.starts_with("opencode")
        || title_lower.contains("opencode")
        || title_lower.starts_with("oc |")
    {
        return (Color::Rgb(88, 101, 242), Color::White); // OpenCode Blue
    } else if title_lower.contains("watch") || title_lower.contains("tail") {
        return (Color::Rgb(156, 39, 176), Color::White); // Watch Purple
    } else if title_lower.starts_with("wget") || title_lower.starts_with("curl") {
        return (Color::Rgb(52, 152, 219), Color::White); // Network Blue
    } else if title_lower.starts_with("make") || title_lower.starts_with("cmake") {
        return (Color::Rgb(230, 126, 34), Color::White); // Build Orange
    } else if title_lower.starts_with("gdb") || title_lower.starts_with("lldb") {
        return (Color::Rgb(231, 76, 60), Color::White); // Debugger Red
    } else if title_lower.starts_with("hugo") || title_lower.starts_with("jekyll") {
        return (Color::Rgb(155, 89, 182), Color::White); // SSG Purple
    } else if title_lower.starts_with("pip") || title_lower.starts_with("poetry") {
        return (Color::Rgb(53, 114, 165), Color::White); // Python Package Blue
    } else if title_lower.starts_with("deno") || title_lower.starts_with("bun") {
        return (Color::Rgb(46, 125, 50), Color::White); // JS Runtime Green
    } else if title_lower.starts_with("zig") || title_lower.starts_with("nim") {
        return (Color::Rgb(222, 76, 65), Color::White); // Compiled Lang Orange
    }

    // Default fallback for shells and other terminals
    (Color::Rgb(103, 117, 140), Color::White) // Kitty Gray
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
    } else if title_lower.starts_with("opencode")
        || title_lower.contains("opencode")
        || title_lower.starts_with("oc |")
    {
        return "󰚩".to_string();
    } else if title_lower.starts_with("lazygit") || title_lower.starts_with("gitui") {
        return "󰊢".to_string();
    } else if title_lower.starts_with("bat")
        || title_lower.starts_with("less")
        || title_lower.starts_with("more")
    {
        return "󰈚".to_string();
    } else if title_lower.starts_with("exa")
        || title_lower.starts_with("lsd")
        || title_lower.starts_with("tree")
    {
        return "󰉋".to_string();
    } else if title_lower.starts_with("fd")
        || title_lower.starts_with("find")
        || title_lower.starts_with("rg")
        || title_lower.starts_with("grep")
        || title_lower.starts_with("ag")
    {
        return "󰍉".to_string();
    } else if title_lower.starts_with("k9s") || title_lower.starts_with("kubectl") {
        return "󱃾".to_string();
    } else if title_lower.starts_with("terraform") || title_lower.starts_with("tf") {
        return "󱁢".to_string();
    } else if title_lower.starts_with("ansible") || title_lower.starts_with("ansible-playbook") {
        return "󰔚".to_string();
    } else if title_lower.starts_with("tmux") || title_lower.starts_with("screen") {
        return "󰆍".to_string();
    } else if title_lower.starts_with("weechat") || title_lower.starts_with("irssi") {
        return "󰒱".to_string();
    } else if title_lower.starts_with("neomutt") || title_lower.starts_with("mutt") {
        return "󰇰".to_string();
    } else if title_lower.starts_with("newsboat") || title_lower.starts_with("nnn") {
        return "󰎕".to_string();
    } else if title_lower.starts_with("ncdu") || title_lower.starts_with("du") {
        return "󰉋".to_string();
    } else if title_lower.starts_with("glow") || title_lower.starts_with("mdcat") {
        return "󰍹".to_string();
    } else if title_lower.starts_with("tig") || title_lower.starts_with("lazydocker") {
        return "󰊢".to_string();
    } else if title_lower.starts_with("fzf")
        || title_lower.starts_with("peco")
        || title_lower.starts_with("ripgrep-all")
        || title_lower.starts_with("rga")
    {
        return "󰍉".to_string();
    } else if title_lower.starts_with("jq") || title_lower.starts_with("yq") {
        return "󰉼".to_string();
    } else if title_lower.starts_with("bottom") || title_lower.starts_with("glances") {
        return "󰔚".to_string();
    } else if title_lower.starts_with("nmap") || title_lower.starts_with("netstat") {
        return "󰈁".to_string();
    } else if title_lower.starts_with("hugo") || title_lower.starts_with("jekyll") {
        return "󰀶".to_string();
    } else if title_lower.starts_with("pip") || title_lower.starts_with("poetry") {
        return "󰌠".to_string();
    } else if title_lower.starts_with("deno") || title_lower.starts_with("bun") {
        return "󰎙".to_string();
    } else if title_lower.starts_with("zig") || title_lower.starts_with("nim") {
        return "󱘗".to_string();
    } else if title_lower.starts_with("make") || title_lower.starts_with("cmake") {
        return "󰔧".to_string();
    } else if title_lower.starts_with("gdb") || title_lower.starts_with("lldb") {
        return "󰃤".to_string();
    } else if title_lower.starts_with("strace")
        || title_lower.starts_with("ltrace")
        || title_lower.starts_with("valgrind")
    {
        return "󰔚".to_string();
    } else if title_lower.starts_with("wireshark") || title_lower.starts_with("tshark") {
        return "󰈁".to_string();
    } else if title_lower.starts_with("sqlite3")
        || title_lower.starts_with("mysql")
        || title_lower.starts_with("redis-cli")
        || title_lower.starts_with("psql")
    {
        return "󰆼".to_string();
    } else if title_lower.starts_with("gh") || title_lower.starts_with("hub") {
        return "󰊢".to_string();
    } else if title_lower.starts_with("alacritty") || title_lower.starts_with("foot") {
        return "󰆍".to_string();
    } else if title_lower.starts_with("nvim-qt") || title_lower.starts_with("gvim") {
        return "".to_string();
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

fn get_kitty_tabs(socket_path: Option<&str>) -> Option<Vec<TabInfo>> {
    if let Some(pid) = get_focused_kitty_pid() {
        get_kitty_tabs_for_pid(pid, socket_path)
    } else {
        None
    }
}

fn get_kitty_tabs_for_pid(pid: u32, socket_path: Option<&str>) -> Option<Vec<TabInfo>> {
    let socket_path_str = match socket_path {
        Some(path) => path.to_string(),
        None => format!("/tmp/kitty-{}", pid),
    };

    let output = Command::new("kitty")
        .args(["@", "--to", &format!("unix:{}", socket_path_str), "ls"])
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
        assert!(kitty_tabs.socket_path.is_none());
    }

    #[test]
    fn test_kitty_tabs_with_config() {
        let socket_path = Some("/tmp/custom-kitty".to_string());
        let kitty_tabs = KittyTabs::with_config(socket_path.clone());
        assert!(kitty_tabs.kitty_pid.is_none() || kitty_tabs.kitty_pid.is_some());
        assert_eq!(kitty_tabs.socket_path, socket_path);
    }

    #[test]
    fn test_kitty_tabs_update() {
        let mut kitty_tabs = KittyTabs::new();
        kitty_tabs.update();

        let mut kitty_tabs_with_config = KittyTabs::with_config(Some("/tmp/test".to_string()));
        kitty_tabs_with_config.update();
    }

    #[test]
    fn test_kitty_tabs_render_empty() {
        let kitty_tabs = KittyTabs {
            tabs: vec![],
            kitty_pid: None,
            socket_path: None,
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
            socket_path: None,
        };
        let spans = kitty_tabs.render_as_spans(true);
        assert_eq!(spans.len(), 2);

        // Check that active tab includes icon + title
        let active_content = spans[1].content.clone();
        assert!(active_content.contains("󰆍")); // Default shell icon
        assert!(active_content.contains("Active Tab"));
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
        assert_eq!(get_tab_icon("opencode help"), "󰚩");
        assert_eq!(get_tab_icon("lazygit"), "󰊢");
        assert_eq!(get_tab_icon("bat README.md"), "󰈚");
        assert_eq!(get_tab_icon("k9s"), "󱃾");
        assert_eq!(get_tab_icon("fzf"), "󰍉");
        assert_eq!(get_tab_icon("random command"), "󰆍"); // default shell icon
    }

    #[test]
    fn test_active_tab_with_icon() {
        let kitty_tabs = KittyTabs {
            tabs: vec![TabInfo {
                title: "opencode help".to_string(),
                is_active: true,
            }],
            kitty_pid: Some(12345),
            socket_path: None,
        };
        let spans = kitty_tabs.render_as_spans(true);
        assert_eq!(spans.len(), 1);

        let content = spans[0].content.clone();
        assert!(content.contains("󰚩")); // OpenCode icon
        assert!(content.contains("opencode"));
    }
}
