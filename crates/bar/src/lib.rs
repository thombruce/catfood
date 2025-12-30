use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Layout},
    prelude::Constraint,
};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

pub mod component_manager;
pub mod components;
pub mod config;
pub mod logging;
pub mod lua_component;
pub mod time_utils;

pub use component_manager::ComponentManager;
pub use components::{LeftBar, MiddleBar, RightBar};

/// Check if bar is already running by checking PID file
pub fn is_bar_running() -> color_eyre::Result<bool> {
    let pid_file_path = get_pid_file_path()?;

    if !pid_file_path.exists() {
        return Ok(false);
    }

    let pid_content = fs::read_to_string(&pid_file_path)?;
    let pid: u32 = pid_content
        .trim()
        .parse()
        .map_err(|_| color_eyre::eyre::eyre!("Invalid PID in PID file"))?;

    // Check if process exists by sending signal 0
    unsafe {
        if libc::kill(pid as i32, 0) == 0 {
            Ok(true) // Process exists and is alive
        } else {
            // Process doesn't exist, remove stale PID file
            let _ = fs::remove_file(&pid_file_path);
            Ok(false)
        }
    }
}

/// Find the catfood-bar executable using multiple strategies
fn find_bar_executable() -> color_eyre::Result<std::path::PathBuf> {
    // Strategy 1: Try PATH first (works for installed packages)
    if let Ok(bar_exe) = which::which("catfood-bar") {
        return Ok(bar_exe);
    }

    // Strategy 2: Try CARGO_BIN_EXE (works during development with cargo run)
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_catfood-bar") {
        let path = std::path::PathBuf::from(path);
        if path.exists() {
            return Ok(path);
        }
    }

    // Strategy 3: Try relative to current executable (development fallback)
    let current_exe = std::env::current_exe()?;
    let bar_exe = current_exe
        .parent()
        .unwrap_or(&current_exe)
        .join("catfood-bar");

    if bar_exe.exists() {
        return Ok(bar_exe);
    }

    // Strategy 4: Try target directories (development fallback)
    let current_dir = std::env::current_dir()?;
    let target_debug = current_dir.join("target/debug/catfood-bar");
    if target_debug.exists() {
        return Ok(target_debug);
    }

    let target_release = current_dir.join("target/release/catfood-bar");
    if target_release.exists() {
        return Ok(target_release);
    }

    Err(color_eyre::eyre::eyre!(
        "Could not find catfood-bar executable.\n\n\
         Please install catfood-bar with one of these methods:\n\
         • cargo install catfood-bar\n\
         • Download from https://github.com/thombruce/catfood/releases\n\n\
         Or ensure it's available in your PATH if already installed."
    ))
}

/// Spawn bar executable in a kitten panel
pub fn spawn_in_panel() {
    // Find the bar executable using robust discovery
    let bar_exe = match find_bar_executable() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Spawn kitten panel directly with proper arguments for security
    // This avoids shell injection risks from special characters in paths
    match Command::new("kitten")
        .arg("panel")
        .arg("--single-instance")
        .arg(&bar_exe)
        .arg("--no-kitten") // Required to prevent spawning additional panels
        .spawn()
    {
        Ok(_child) => {
            // Give panel a moment to start then exit parent
            // The child process continues running independently
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to spawn kitten panel: {}", e);
            eprintln!(
                "Make sure Kitty is installed and you're running this in a Kitty environment."
            );
            std::process::exit(1);
        }
    }
}

/// Handle common bar CLI logic: check if running and optionally spawn in panel
/// Returns true if spawning in panel (process will exit via spawn_in_panel),
/// false if should continue with direct execution
pub fn handle_bar_cli(no_kitten: bool) -> bool {
    if !no_kitten {
        // Check if already running
        if let Ok(true) = is_bar_running() {
            eprintln!("catfood-bar is already running");
            std::process::exit(1);
        }

        // Spawn in panel - this function will exit the process
        spawn_in_panel();
        // This line is unreachable, but required for type compatibility
        unreachable!("spawn_in_panel() should have exited the process")
    } else {
        false // Continue with direct execution (--no-kitten case)
    }
}

pub fn run_bar() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Create PID file at bar startup (not in parent)
    if let Err(e) = create_pid_file() {
        eprintln!("Failed to create PID file: {}", e);
        return Err(e);
    }

    // Initialize Tokio runtime
    let rt = Runtime::new()?;

    let result = rt.block_on(async {
        let terminal = ratatui::init();
        let app_result = App::new()?.run_async(terminal).await;
        ratatui::restore();
        app_result
    });

    // Clean up PID file on exit
    let _ = remove_pid_file();

    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    component_manager: ComponentManager,
    left_bar: LeftBar,
    middle_bar: MiddleBar,
    right_bar: RightBar,
    reload_rx: mpsc::Receiver<()>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> color_eyre::Result<Self> {
        let component_manager = ComponentManager::new()?;
        let (reload_tx, reload_rx) = mpsc::channel(10);

        // Start file watcher
        Self::start_config_watcher(reload_tx)?;

        Ok(Self {
            running: true,
            component_manager,
            left_bar: LeftBar::new()?,
            middle_bar: MiddleBar::new()?,
            right_bar: RightBar::new()?,
            reload_rx,
        })
    }

    /// Start the configuration file watcher
    fn start_config_watcher(reload_tx: mpsc::Sender<()>) -> color_eyre::Result<()> {
        let config_path =
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".config")
                .join("catfood")
                .join("bar.json");

        tokio::spawn(async move {
            use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
            use std::time::Duration;

            let (tx, mut rx) = tokio::sync::mpsc::channel(10);

            // Create watcher with proper error handling
            let mut watcher = match RecommendedWatcher::new(
                move |res| {
                    if let Ok(event) = res {
                        let _ = tx.blocking_send(event);
                    }
                },
                NotifyConfig::default().with_poll_interval(Duration::from_secs(1)),
            ) {
                Ok(w) => w,
                Err(e) => {
                    logging::log_file_watcher_error(&format!(
                        "Failed to create file watcher: {}",
                        e
                    ));
                    return;
                }
            };

            // Watch the config directory
            if let Some(parent) = config_path.parent()
                && let Err(e) = watcher.watch(parent, RecursiveMode::NonRecursive)
            {
                logging::log_file_watcher_error(&format!(
                    "Failed to watch config directory: {}",
                    e
                ));
                return;
            }

            while let Some(event) = rx.recv().await {
                use notify::EventKind;

                // Check if the event is related to our config file
                if let Some(path) = event.paths.first()
                    && path == &config_path
                    && matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
                    && let Err(e) = reload_tx.send(()).await
                {
                    logging::log_file_watcher_error(&format!(
                        "Failed to send reload signal: {}",
                        e
                    ));
                    break;
                }
            }
        });

        Ok(())
    }

    /// Run the application's main loop.
    pub async fn run_async(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            tokio::select! {
                _ = self.reload_rx.recv() => {
                    // Handle config reload
                    if let Err(e) = self.component_manager.reload() {
                        logging::log_config_error(&format!("Failed to reload configuration: {}", e));
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(333)) => {
                    // Normal update cycle
                    self.update_components();
                    terminal.draw(|frame| self.render(frame))?;
                    self.handle_crossterm_events()?;
                }
            }
        }
        Ok(())
    }

    fn update_components(&mut self) {
        if let Err(e) = self.component_manager.update() {
            logging::log_system_error("Component Manager", &format!("{}", e));
        }
        if let Err(e) = self.left_bar.update() {
            logging::log_system_error("Left Bar", &format!("{}", e));
        }
        if let Err(e) = self.middle_bar.update() {
            logging::log_system_error("Middle Bar", &format!("{}", e));
        }
        if let Err(e) = self.right_bar.update() {
            logging::log_system_error("Right Bar", &format!("{}", e));
        }
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.area());

        self.left_bar
            .render(frame, layout[0], &self.component_manager);
        self.middle_bar
            .render(frame, layout[1], &self.component_manager);
        self.right_bar
            .render(frame, layout[2], &self.component_manager);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(333))? {
            match event::read()? {
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
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

/// Get the PID file path (same as in catfood crate)
fn get_pid_file_path() -> color_eyre::Result<PathBuf> {
    let data_dir = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.local/share", home)
    });

    let catfood_dir = PathBuf::from(data_dir).join("catfood");
    fs::create_dir_all(&catfood_dir)?;

    Ok(catfood_dir.join("bar.pid"))
}

/// Remove PID file
fn remove_pid_file() -> color_eyre::Result<()> {
    let pid_file_path = get_pid_file_path()?;

    if pid_file_path.exists() {
        fs::remove_file(&pid_file_path)?;
    }

    Ok(())
}

/// Create PID file with current process ID
fn create_pid_file() -> color_eyre::Result<()> {
    let pid_file_path = get_pid_file_path()?;
    let pid = std::process::id();

    let mut file = fs::File::create(&pid_file_path)?;
    writeln!(file, "{}", pid)?;

    Ok(())
}
