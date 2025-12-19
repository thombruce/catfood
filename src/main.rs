// NOTE: On how to think about the structure of a ratatui application...
// We have got:
// 1. State (see pub struct App and initial run setup)
// 2. Controller/mutations (see the while loop which also calls the draw function)
// 3. Renderer (see fn render)
// When we think about separating this out into modules... a naive approach might be to
// take a sort of single file component approach, with each component defining its own
// needs and therefore having many separate loops. Better would be to handle state more
// effectively (think like Pinia in Nuxt). Ratatui in fact has a suggested means for
// appropriately handling state that I should look at.

use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Layout},
    prelude::Constraint,
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};
use regex::Regex;
use serde::Deserialize;
use std::{io, process::Command, time::Duration};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default, Clone)]
pub struct App {
    /// Is the application running?
    running: bool,
    time: String,
    volume: String,
    brightness: String,
    bat_percent: String,
    cpu: String,
    ram: String,
    workspaces: Vec<String>,
    active_workspace: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;

        // CPU setup
        let mut s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        // Wait a bit because CPU usage is based on diff.
        // TODO: Sleep affects startup? It's also only needed for the CPU widget, right?
        // This should be isolated somehow. Should it also be waited on before each CPU update?
        // It also isn't strictly necessary, since we're refreshing the info every draw anyway;
        // I've commented this out for now.
        // std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        // TODO: Move me! Ideally each widget should be moved into its own file and assembled here.
        // TODO: Document me! We need better comments to describe what's going on. This doesn't get
        // the battery state at all, for instance, it just gets the first found instance of a
        // battery.
        let manager = battery::Manager::new()?;
        let mut battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e.into());
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(io::Error::from(io::ErrorKind::NotFound).into());
            }
        };

        while self.running {
            // TODO: Time updates appear inconsistently timed. The half-second timer can result in
            // the string value being updated twice inside of the same half-second. This has the
            // effect of making an actual second look... wrong.
            self.time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            // TODO: Below appears slow as it is run on the 500 millisecond timer
            // but the volume is updated with a keypress. It might also be updated via
            // graphical UI or by scripts. It should be updated accordingly whenever its value is
            // changed. So then what event do we listen out for?
            // BEST solution would be to listen out for volume change events, however crossterm
            // does not appear to have this capability..? Presumably the sound control systems we
            // have installed do output some kind of information we could listen to somewhere?
            self.volume = get_system_volume().unwrap().to_string();

            self.brightness = get_system_brightness().unwrap().to_string();

            self.bat_percent = ((battery.state_of_charge().value * 100.0) as i32).to_string();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
            manager.refresh(&mut battery)?;

            // TODO: On the other hand, these are refreshed too quickly. We need to handle the
            // refresh rate of CPU and RAM separately from time and volume.
            s.refresh_cpu_all();
            s.refresh_memory();

            // TODO: We don't necessarily need to reobtain total_memory. We can store this value
            // permantely somewhere.
            let mem_percent: u32 =
                (s.used_memory() as f64 / s.total_memory() as f64 * 100.0) as u32;
            // TODO: Consider storing numeric values as their numeric types. String conversion
            // should be handled within the renderer scope.
            self.ram = mem_percent.to_string();

            // TODO: iter can be created once. Move this out of loop.
            let iter = s.cpus().iter();
            // TODO: count can be counted once. Move out of loop.
            let count = iter.len() as f32;
            // sum and avg must be recalculated following CPU refresh.
            let sum = iter.fold(0.0, |acc, x| acc + x.cpu_usage());
            let avg: u32 = (sum / count) as u32;
            self.cpu = avg.to_string();

            self.workspaces = get_workspaces().expect("workspaces not found");
            self.active_workspace = get_active_workspace().expect("active workspace not found");
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.area());

        let sep_span = Span::raw(" | ");
        let space_span = Span::raw(" ");
        // TODO: Consider actual usefulness of space_span. We can just write space after icons
        // below. Perhaps it is useful though as a separator for dividing components from one
        // another.

        let workspaces = self
            .workspaces
            .iter()
            .map(|w| {
                if w == &self.active_workspace {
                    Span::raw(format!(" {} ", w))
                        .bg(Color::White)
                        .fg(Color::Black)
                } else {
                    Span::raw(format!(" {} ", w))
                }
            })
            .collect::<Vec<Span>>();

        frame.render_widget(Line::from(workspaces).left_aligned(), layout[0]);

        let time_span = Span::raw(&self.time);

        frame.render_widget(
            Paragraph::new(time_span).centered().fg(Color::White),
            layout[1],
        );

        // TODO: Temp
        // TODO: WiFi
        // TODO: VPN

        let cpu_icon = Span::raw("󰻠 ".to_owned());
        let cpu_span = Span::raw(self.cpu.clone() + "%");
        let ram_icon = Span::raw("󰍛 ".to_owned());
        let ram_span = Span::raw(self.ram.clone() + "%");

        let brightness_icon = Span::raw("󰃠 ".to_owned()); // .green();
        let brightness_span = Span::raw(self.brightness.clone());

        // TODO: As below, we can conditionally modify icon and color
        let vol_icon = Span::raw("󰕾 ".to_owned()); // .green();
        let vol_span = Span::raw(self.volume.clone() + "%"); // .green();

        // TODO: bat_icon being separate means we should be able to use the bat_percent value to
        // conditionally set half-full and close-to-empty icons, and also conditionally apply
        // different colors (though color-mode should be configurable either via config file or
        // command line argument)
        let bat_icon = Span::raw("󰁹 ".to_owned()); // .green();
        let bat_span = Span::raw(self.bat_percent.clone() + "%"); // .green();

        let widget_line = Line::from(vec![
            cpu_icon,
            cpu_span,
            space_span.clone(),
            ram_icon,
            ram_span,
            sep_span.clone(),
            brightness_icon,
            brightness_span,
            space_span.clone(),
            vol_icon,
            vol_span,
            sep_span.clone(),
            bat_icon,
            bat_span,
            space_span.clone(),
        ]);

        frame.render_widget(
            Paragraph::new(widget_line)
                .right_aligned()
                // .fg(Color::Rgb(189, 43, 174)),
                .fg(Color::White),
            layout[2],
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(333))? {
            // TODO: In fact, the bar widget won't be able to receive keypress events at all...
            // correct? Since it won't have focused state? Perhaps it can respond to keypresses
            // without focus but... I don't *think* we want that. I think we want the widgets to
            // update on tick (clock) or in response to other state changes (volume, battery, VPN,
            // etc.)
            match event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

#[derive(Deserialize, Debug)]
struct Workspace {
    id: i32,
}

fn get_workspaces() -> Option<Vec<String>> {
    let output = Command::new("hyprctl")
        .args(["workspaces", "-j"]) // -j outputs in json, which ought to be easier to work with
        .output()
        .expect("failed to get workspaces");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let json: Vec<Workspace> =
            serde_json::from_str(stdout).expect("failed to parse workspaces");

        return Some(json.iter().map(|j| j.id.clone().to_string()).collect());
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
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
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    None
}

fn get_system_volume() -> Option<i32> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .expect("failed to get volume");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        let parts: Vec<&str> = stdout.trim().split_whitespace().collect();

        if let Some(volume_str) = parts.last() {
            if let Ok(volume) = volume_str.parse::<f32>() {
                return Some((volume * 100.0) as i32); // as percentage
            }
        }

        eprintln!("Failed to parse volume from output: {}", stdout);
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    None
}

fn get_system_brightness() -> Option<String> {
    let output = Command::new("brightnessctl")
        .output()
        .expect("failed to get brightness");

    if output.status.success() {
        let brightness_str = str::from_utf8(&output.stdout).unwrap();

        // TODO: This should be a constant rather than recreated here on
        // every loop.
        let re = Regex::new(r"\d+%").unwrap();

        if let Some(brightness) = re.find(brightness_str).map(|m| m.as_str()) {
            return Some(brightness.to_string()); // as percentage
        }

        eprintln!("Failed to parse volume from output: {}", brightness_str);
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    None
}
