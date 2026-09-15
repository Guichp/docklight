mod docker;

use docker::ResourceSummary;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

// App struct must be created before main()
struct App {
    should_quit: bool,
    resources: Vec<ResourceSummary>,
    list_state: ListState,
    message: String,
}

fn main() -> io::Result<()> {
    let mut app = App {
        should_quit: false,
        resources: vec![
            ResourceSummary {
                name: String::from("Images"),
                count: 12,
                reclaimable_bytes: 4_200_000_000,
                // this means -> Images: 12 resources, 4200MB reclaimable
            },
            ResourceSummary {
                name: String::from("Containers"),
                count: 3,
                reclaimable_bytes: 850_000_000,
            },
        ],
        list_state: ListState::default(),
        message: String::from("Select a resource and press Enter"),
    };

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // main loop for the app
    while !app.should_quit {
        terminal.draw(|frame| {
            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // gives the header three rows
                    Constraint::Min(1),    // lets the content use the remaining space
                    Constraint::Length(1), // gives the footer one row
                ])
                .split(frame.area());

            let header =
                Paragraph::new("Docker Cleaner").block(Block::default().borders(Borders::ALL));

            let items = app
                .resources
                .iter()
                .map(|resource| ListItem::new(resource.display_text()))
                .collect::<Vec<_>>();

            let content = List::new(items)
                .block(Block::default().title("Resources").borders(Borders::ALL))
                .highlight_symbol("> ");

            let footer = Paragraph::new(app.message.as_str());

            frame.render_widget(header, areas[0]);
            frame.render_stateful_widget(content, areas[1], &mut app.list_state);
            frame.render_widget(footer, areas[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key.code);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

impl App {
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.move_selection(1),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection(-1),
            KeyCode::Enter => {
                let index = self.list_state.selected().unwrap_or(0);
                let resource = &self.resources[index];

                self.message = format!("Selected {}", resource.name);
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
