use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Layout},
    prelude::Constraint,
};
use std::time::Duration;
use tokio::runtime::Runtime;

mod components;
use components::{MiddleBar, SystemBar, Workspaces};

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
    workspaces: Workspaces,
    middle_bar: MiddleBar,
    system_bar: SystemBar,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            running: true,
            workspaces: Workspaces::new(),
            middle_bar: MiddleBar::new()?,
            system_bar: SystemBar::new()?,
        })
    }

    /// Run the application's main loop.
    pub async fn run_async(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            self.update_components();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn update_components(&mut self) {
        self.workspaces.update();
        if let Err(e) = self.middle_bar.update() {
            eprintln!("Error updating middle bar: {}", e);
        }
        if let Err(e) = self.system_bar.update() {
            eprintln!("Error updating system bar: {}", e);
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

        self.workspaces.render(frame, layout[0]);
        self.middle_bar.render(frame, layout[1]);
        self.system_bar.render(frame, layout[2]);
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
