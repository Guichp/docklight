use crate::docker::ResourceSummary;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

// App struct must be created before main()
pub struct App {
    pub should_quit: bool,
    pub resources: Vec<ResourceSummary>,
    pub list_state: ListState,
    pub message: String,
}

impl App {
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.move_selection(1),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection(-1),
            KeyCode::Enter => {
                let index = self.list_state.selected().unwrap_or(0);
                let resource = &self.resources[index];

                self.message = format!("Selected {}", resource.name());
            }
            _ => {}
        }
    }

    fn move_selection(&mut self, amount: isize) {
        let current = self.list_state.selected().unwrap_or(0);
        let last = self.resources.len().saturating_sub(1);

        let next = if amount.is_negative() {
            current.saturating_sub(amount.unsigned_abs())
        } else {
            current.saturating_add(amount as usize).min(last)
        };

        self.list_state.select(Some(next))
    }
}
