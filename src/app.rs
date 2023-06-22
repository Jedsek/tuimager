use crate::utils::clear_screen;
use std::{error, path::PathBuf};
use tui::widgets::ListState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub images: StatefulList<PathBuf>,
    pub view_mode: bool,
}

#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            images: StatefulList::default(),
            view_mode: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(image_paths: Vec<PathBuf>) -> Self {
        Self {
            images: StatefulList::with_items(image_paths),
            ..Self::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn on_up(&mut self) {
        if self.view_mode {
            clear_screen();
        }

        self.images.prev_item();
    }

    pub fn on_down(&mut self) {
        if self.view_mode {
            clear_screen();
        }
        self.images.next_item();
    }

    pub fn on_esc(&mut self) {
        if self.view_mode {
            self.toggle_view();
        } else {
            self.images.unselect();
        }
    }

    pub fn toggle_view(&mut self) {
        if self.view_mode {
            clear_screen()
        }
        self.view_mode = !self.view_mode;
    }
}
impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    fn prev_item(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            None => 0,
            Some(i) if i == 0 => self.items.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(i))
    }

    pub fn next_item(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            None => 0,
            Some(i) if i >= self.items.len() - 1 => 0,
            Some(i) => i + 1,
        };
        self.state.select(Some(i))
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
