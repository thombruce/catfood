use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::Deserialize;
use std::process::Command;

use crate::logging;

#[derive(Deserialize, Debug)]
struct Workspace {
    id: i32,
}

#[derive(Debug, Default, Clone)]
pub struct Workspaces {
    pub workspaces: Vec<String>,
    pub active_workspace: String,
}

impl Workspaces {
    pub fn new() -> Self {
        Self {
            workspaces: get_workspaces().unwrap_or_default(),
            active_workspace: get_active_workspace().unwrap_or_default(),
        }
    }

    pub fn update(&mut self) {
        self.workspaces = get_workspaces().unwrap_or_default();
        self.active_workspace = get_active_workspace().unwrap_or_default();
    }

    pub fn render(&self) -> Vec<Span<'_>> {
        let rainbow_colors = [
            Color::Red,      // 1
            Color::Yellow,   // 2
            Color::Green,    // 3
            Color::Cyan,     // 4
            Color::Blue,     // 5
            Color::Magenta,  // 6
            Color::LightRed, // 7
        ];

        self.workspaces
            .iter()
            .map(|w| {
                if w == &self.active_workspace {
                    if let Ok(workspace_num) = w.parse::<usize>() {
                        let color_index = (workspace_num - 1) % rainbow_colors.len();
                        let bg_color = rainbow_colors[color_index];
                        // Use black text for better readability on all colored backgrounds
                        Span::raw(format!(" {} ", w)).bg(bg_color).fg(Color::Black)
                    } else {
                        // Fallback for non-numeric workspace names
                        Span::raw(format!(" {} ", w))
                            .bg(Color::White)
                            .fg(Color::Black)
                    }
                } else if let Ok(workspace_num) = w.parse::<usize>() {
                    let color_index = (workspace_num - 1) % rainbow_colors.len();
                    let color = rainbow_colors[color_index];
                    Span::raw(format!(" {} ", w)).fg(color)
                } else {
                    Span::raw(format!(" {} ", w))
                }
            })
            .collect::<Vec<Span>>()
    }
}

fn get_workspaces() -> Option<Vec<String>> {
    let output = Command::new("hyprctl")
        .args(["workspaces", "-j"])
        .output()
        .expect("failed to get workspaces");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let json: Vec<Workspace> =
            serde_json::from_str(stdout).expect("failed to parse workspaces");

        return Some(json.iter().map(|j| j.id.clone().to_string()).collect());
    } else {
        logging::log_component_error(
            "WORKSPACES",
            str::from_utf8(&output.stderr).unwrap_or("unknown error"),
        );
    }

    None
}

fn get_active_workspace() -> Option<String> {
    let output = Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()
        .expect("failed to get active workspace");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let json: Workspace =
            serde_json::from_str(stdout).expect("failed to parse active workspace");

        return Some(json.id.clone().to_string());
    } else {
        logging::log_component_error(
            "WORKSPACES",
            str::from_utf8(&output.stderr).unwrap_or("unknown error"),
        );
    }

    None
}
