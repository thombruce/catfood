use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Layout},
    prelude::Constraint,
};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

mod component_manager;
mod components;
mod config;

use component_manager::ComponentManager;
use components::{LeftBar, MiddleBar, RightBar};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Initialize Tokio runtime
    let rt = Runtime::new()?;

    rt.block_on(async {
        let terminal = ratatui::init();
        let result = App::new()?.run_async(terminal).await;
        ratatui::restore();
        result
    })
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
        
        println!("Configuration hot-reload enabled. Edit ~/.config/catfoodBar/config.json to see changes live!");
        
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
        let config_path = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("catfoodBar")
            .join("config.json");
        
        tokio::spawn(async move {
            use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config as NotifyConfig};
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
                    eprintln!("Failed to create file watcher: {}", e);
                    return;
                }
            };
            
            // Watch the config directory
            if let Some(parent) = config_path.parent() {
                if let Err(e) = watcher.watch(parent, RecursiveMode::NonRecursive) {
                    eprintln!("Failed to watch config directory: {}", e);
                    return;
                }
            }
            
            while let Some(event) = rx.recv().await {
                use notify::EventKind;
                
                // Check if the event is related to our config file
                if let Some(path) = event.paths.first() {
                    if path == &config_path {
                        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                            println!("Configuration file changed, reloading...");
                            if let Err(e) = reload_tx.send(()).await {
                                eprintln!("Failed to send reload signal: {}", e);
                                break;
                            }
                        }
                    }
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
                        eprintln!("Failed to reload configuration: {}", e);
                    } else {
                        println!("Configuration reloaded successfully!");
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
            eprintln!("Error updating components: {}", e);
        }
        if let Err(e) = self.left_bar.update() {
            eprintln!("Error updating left bar: {}", e);
        }
        if let Err(e) = self.middle_bar.update() {
            eprintln!("Error updating middle bar: {}", e);
        }
        if let Err(e) = self.right_bar.update() {
            eprintln!("Error updating right bar: {}", e);
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

        self.left_bar.render(frame, layout[0], &self.component_manager);
        self.middle_bar.render(frame, layout[1], &self.component_manager);
        self.right_bar.render(frame, layout[2], &self.component_manager);
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
