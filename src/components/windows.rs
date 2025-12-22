use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::Deserialize;
use std::process::Command;

use crate::logging;

#[derive(Deserialize, Debug)]
struct Window {
    address: String,
    class: String,
    workspace: Workspace,
}

#[derive(Deserialize, Debug)]
struct ActiveWindow {
    address: String,
}

#[derive(Deserialize, Debug)]
struct Workspace {
    id: i32,
}

#[derive(Debug, Clone)]
pub struct WindowInfo {
    address: String,
    icon: String,
}

#[derive(Debug, Default, Clone)]
pub struct Windows {
    pub windows: Vec<WindowInfo>,
    active_window: String,
}

impl Windows {
    pub fn new() -> Self {
        let (windows, active_window) = get_windows().unwrap_or_default();
        Self {
            windows,
            active_window,
        }
    }

    pub fn update(&mut self) {
        let (windows, active_window) = get_windows().unwrap_or_default();
        self.windows = windows;
        self.active_window = active_window;
    }

    pub fn render(&self) -> Vec<Span<'_>> {
        self.windows
            .iter()
            .map(|w| {
                if w.address == self.active_window {
                    // Focused window: white background with black text
                    Span::raw(format!(" {} ", w.icon))
                        .bg(Color::White)
                        .fg(Color::Black)
                } else {
                    // Unfocused window: white text on default background
                    Span::raw(format!(" {} ", w.icon)).fg(Color::White)
                }
            })
            .collect::<Vec<Span>>()
    }
}

fn get_windows() -> Option<(Vec<WindowInfo>, String)> {
    // Get all windows
    let clients_output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .expect("failed to get clients");

    if !clients_output.status.success() {
        logging::log_component_error(
            "WINDOWS",
            str::from_utf8(&clients_output.stderr).unwrap_or("unknown error"),
        );
        return None;
    }

    let clients_stdout = str::from_utf8(&clients_output.stdout).unwrap();
    let windows: Vec<Window> =
        serde_json::from_str(clients_stdout).expect("failed to parse windows");

    // Get active window
    let active_output = Command::new("hyprctl")
        .args(["activewindow", "-j"])
        .output()
        .expect("failed to get active window");

    let active_address = if active_output.status.success() {
        let active_stdout = str::from_utf8(&active_output.stdout).unwrap();
        let active_window: ActiveWindow =
            serde_json::from_str(active_stdout).unwrap_or(ActiveWindow {
                address: String::new(),
            });
        active_window.address
    } else {
        String::new()
    };

    let window_infos = windows
        .iter()
        .filter(|w| w.workspace.id > 0) // Filter out special workspaces
        .map(|w| WindowInfo {
            address: w.address.clone(),
            icon: get_app_icon(&w.class),
        })
        .collect();

    Some((window_infos, active_address))
}

fn get_app_icon(class: &str) -> String {
    match class.to_lowercase().as_str() {
        // Browsers
        "firefox" | "firefox-developer-edition" => "󰈹".to_string(),
        "google-chrome" | "chrome" => "󰊯".to_string(),
        "chromium" => "󰊯".to_string(),
        "brave-browser" => "󰖟".to_string(),
        "librewolf" => "󰈹".to_string(),
        "vivaldi" => "󰖟".to_string(),
        "opera" => "󰖟".to_string(),
        "edge" => "󰇩".to_string(),
        "helium" => "󰖟".to_string(),

        // Terminal & Editors
        "kitty" => "󰄛".to_string(),
        "alacritty" => "󰆍".to_string(),
        "gnome-terminal" => "󰆍".to_string(),
        "konsole" => "󰆍".to_string(),
        "xterm" => "󰆍".to_string(),
        "neovide" => "".to_string(),
        "nvim" | "neovim" => "".to_string(),
        "vim" => "".to_string(),
        "emacs" => "󰍹".to_string(),
        "code" | "code-oss" => "󰨞".to_string(),
        "sublime_text" => "󰅪".to_string(),

        // PDF & Document Viewers
        "zathura" => "󰈦".to_string(),
        "evince" => "󰈦".to_string(),
        "okular" => "󰈦".to_string(),
        "qpdfview" => "󰈦".to_string(),
        "mupdf" => "󰈦".to_string(),

        // Image Viewers
        "qview" => "󰋩".to_string(),
        "feh" => "󰋩".to_string(),
        "nomacs" => "󰋩".to_string(),
        "gwenview" => "󰋩".to_string(),
        "eog" => "󰋩".to_string(),
        "sxiv" => "󰋩".to_string(),

        // Video Players
        "mpv" => "󰐹".to_string(),
        "vlc" => "󰕼".to_string(),
        "smplayer" => "󰐹".to_string(),
        "celluloid" => "󰐹".to_string(),

        // Music & Audio
        "spotify" => "󰓇".to_string(),
        "rhythmbox" => "󰓇".to_string(),
        "audacious" => "󰓇".to_string(),
        "cmus" => "󰓇".to_string(),
        "ncmpcpp" => "󰓇".to_string(),

        // Graphics & Design
        "gimp" => "󰏘".to_string(),
        "aseprite" => "󰆧".to_string(),
        "inkscape" => "󰝫".to_string(),
        "blender" => "󰂫".to_string(),
        "krita" => "󰏘".to_string(),
        "obs" => "󰕀".to_string(),

        // Communication
        "discord" => "󰙯".to_string(),
        "telegramdesktop" | "telegram" => "󰈨".to_string(),
        "slack" => "󰒱".to_string(),
        "signal" => "󰍦".to_string(),
        "thunderbird" => "󰇰".to_string(),
        "geary" => "󰇰".to_string(),

        // File Managers
        "thunar" => "󰉋".to_string(),
        "dolphin" => "󰉋".to_string(),
        "nautilus" => "󰉋".to_string(),
        "pcmanfm" => "󰉋".to_string(),
        "ranger" => "󰉋".to_string(),
        "lf" => "󰉋".to_string(),

        // System Tools
        "htop" | "btop" => "󰔚".to_string(),
        "nvtop" => "󰍛".to_string(),
        "pavucontrol" => "󰝚".to_string(),
        "networkmanager_dmenu" => "󰤨".to_string(),

        // Office & Productivity
        "libreoffice-writer" => "󰏪".to_string(),
        "libreoffice-calc" => "󰈛".to_string(),
        "libreoffice-impress" => "󰎧".to_string(),
        "onlyoffice-desktopeditors" => "󰏪".to_string(),

        // Development Tools
        "postman" => "󰮮".to_string(),
        "insomnia" => "󰘦".to_string(),
        "gitkraken" => "󰊢".to_string(),
        "figma-linux" => "󰿭".to_string(),
        "wine" | "winecfg" => "󰡶".to_string(),

        // Games
        "steam" => "󰓓".to_string(),
        "lutris" => "󰮭".to_string(),
        "heroic" => "󰔑".to_string(),
        "minecraft" => "󰍳".to_string(),

        // Generic fallbacks
        // TODO: I've commented these out because there is no way of determining the nature of an
        // application given that we only retrieve its class (application name).
        // "browser" => "󰖟".to_string(),
        // "terminal" => "󰆍".to_string(),
        // "editor" => "".to_string(),
        // "file_manager" => "󰉋".to_string(),
        // "music_player" => "󰓇".to_string(),
        // "video_player" => "󰐹".to_string(),
        // "image_viewer" => "󰋩".to_string(),

        // Default fallback
        _ => "󰍜".to_string(),
    }
}
