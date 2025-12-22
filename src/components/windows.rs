use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::Deserialize;
use std::process::Command;

use crate::logging;

#[derive(Deserialize, Debug)]
struct Window {
    address: String,
    class: String,
    title: String,
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
    class: String,
    title: String,
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
                    // Focused window: brand color background with appropriate text color
                    let (bg_color, fg_color) = get_brand_color(&w.class, &w.title);
                    Span::raw(format!(" {} ", w.icon)).bg(bg_color).fg(fg_color)
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
            icon: get_app_icon(&w.class, &w.title),
            class: w.class.clone(),
            title: w.title.clone(),
        })
        .collect();

    Some((window_infos, active_address))
}

fn get_app_icon(class: &str, title: &str) -> String {
    // First check title for terminal applications with specific commands
    let title_lower = title.to_lowercase();
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
    }

    // Fall back to class-based detection
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

        // Terminal Emulators (fallback when title doesn't match specific commands)
        "kitty" => "󰄛".to_string(),
        "alacritty" => "󰆍".to_string(),
        "gnome-terminal" => "󰆍".to_string(),
        "konsole" => "󰆍".to_string(),
        "xterm" => "󰆍".to_string(),

        // Standalone Editor Applications (GUI-based)
        "neovide" => "".to_string(),
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

        // File Managers (GUI-based)
        "thunar" => "󰉋".to_string(),
        "dolphin" => "󰉋".to_string(),
        "nautilus" => "󰉋".to_string(),
        "pcmanfm" => "󰉋".to_string(),

        // System Tools (GUI-based)
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

fn get_brand_color(class: &str, title: &str) -> (Color, Color) {
    // First check title for terminal applications with specific commands
    let title_lower = title.to_lowercase();
    let class_lower = class.to_lowercase();

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
    }

    // Fall back to class-based colors
    match class_lower.as_str() {
        // Browsers
        "firefox" | "firefox-developer-edition" | "librewolf" => {
            (Color::Rgb(255, 119, 0), Color::Black)
        } // Firefox Orange
        "google-chrome" | "chrome" | "chromium" => (Color::Rgb(66, 133, 244), Color::Black), // Google Blue
        "brave-browser" => (Color::Rgb(250, 72, 41), Color::White), // Brave Red
        "vivaldi" | "opera" => (Color::Rgb(235, 90, 70), Color::White), // Vivaldi/Opera Red
        "edge" => (Color::Rgb(0, 120, 215), Color::White),          // Edge Blue
        "helium" => (Color::Rgb(0, 184, 169), Color::White),        // Helium Teal

        // Terminal Emulators
        "kitty" => (Color::Rgb(103, 117, 140), Color::White), // Kitty Gray
        "alacritty" | "gnome-terminal" | "konsole" | "xterm" => {
            (Color::Rgb(46, 52, 64), Color::White)
        } // Terminal Dark

        // GUI Editors
        "neovide" => (Color::Rgb(0, 107, 84), Color::White), // Neovim Green
        "code" | "code-oss" => (Color::Rgb(27, 127, 243), Color::White), // VS Code Blue
        "sublime_text" => (Color::Rgb(255, 93, 0), Color::White), // Sublime Orange

        // PDF Viewers
        "zathura" | "evince" | "okular" | "qpdfview" | "mupdf" => {
            (Color::Rgb(198, 40, 40), Color::White)
        } // PDF Red

        // Image Viewers
        "qview" | "feh" | "nomacs" | "gwenview" | "eog" | "sxiv" => {
            (Color::Rgb(156, 39, 176), Color::White)
        } // Image Purple

        // Video Players
        "mpv" | "vlc" | "smplayer" | "celluloid" => (Color::Rgb(237, 101, 46), Color::White), // Video Orange

        // Music Players
        "spotify" | "rhythmbox" | "audacious" => (Color::Rgb(29, 185, 84), Color::White), // Music Green

        // Graphics & Design
        "gimp" | "krita" => (Color::Rgb(103, 72, 145), Color::White), // GIMP/Krita Purple
        "aseprite" => (Color::Rgb(255, 255, 255), Color::Black),      // Aseprite White
        "inkscape" => (Color::Rgb(0, 116, 178), Color::White),        // Inkscape Blue
        "blender" => (Color::Rgb(245, 129, 49), Color::White),        // Blender Orange
        "obs" => (Color::Rgb(146, 52, 220), Color::White),            // OBS Purple

        // Communication
        "discord" => (Color::Rgb(88, 101, 242), Color::White), // Discord Blue
        "telegramdesktop" | "telegram" => (Color::Rgb(39, 156, 204), Color::White), // Telegram Blue
        "slack" => (Color::Rgb(254, 0, 84), Color::White),     // Slack Red
        "signal" => (Color::Rgb(83, 189, 238), Color::White),  // Signal Blue
        "thunderbird" | "geary" => (Color::Rgb(0, 112, 193), Color::White), // Email Blue

        // File Managers (GUI)
        "thunar" | "dolphin" | "nautilus" | "pcmanfm" => (Color::Rgb(41, 128, 185), Color::White), // FM Blue

        // System Tools (GUI)
        "nvtop" => (Color::Rgb(0, 173, 181), Color::White), // GPU Monitor Teal
        "pavucontrol" => (Color::Rgb(233, 84, 32), Color::White), // Audio Control Orange
        "networkmanager_dmenu" => (Color::Rgb(0, 184, 169), Color::White), // Network Teal

        // Office
        "libreoffice-writer" | "onlyoffice-desktopeditors" => {
            (Color::Rgb(18, 52, 86), Color::White)
        } // Office Blue
        "libreoffice-calc" => (Color::Rgb(43, 87, 135), Color::White), // Calc Green
        "libreoffice-impress" => (Color::Rgb(233, 63, 51), Color::White), // Impress Red

        // Development Tools
        "postman" => (Color::Rgb(255, 89, 94), Color::White), // Postman Orange
        "insomnia" => (Color::Rgb(148, 66, 156), Color::White), // Insomnia Purple
        "gitkraken" => (Color::Rgb(64, 84, 178), Color::White), // GitKraken Blue
        "figma-linux" => (Color::Rgb(0, 112, 243), Color::White), // Figma Blue
        "wine" | "winecfg" => (Color::Rgb(143, 31, 35), Color::White), // Wine Red

        // Games
        "steam" => (Color::Rgb(0, 47, 71), Color::White), // Steam Dark Blue
        "lutris" => (Color::Rgb(201, 32, 44), Color::White), // Lutris Red
        "heroic" => (Color::Rgb(162, 49, 162), Color::White), // Heroic Purple
        "minecraft" => (Color::Rgb(46, 125, 50), Color::White), // Minecraft Green

        // Default
        _ => (Color::Gray, Color::White),
    }
}
