use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::io;

// App struct must be created before main()
struct App {
    should_quit: bool,
    resources: Vec<ResourceSummary>,
}

// Struct representing one Docker resource category, such as images or containers
struct ResourceSummary {
    name: String,        // resource name, such as "Images"
    count: u32,          // number of resources in that category, such as 12 images
    reclaimable_mb: u64, // estimated disk space that could be recovered, like 4200 MB
}

fn main() -> io::Result<()> {
    let mut app = App {
        should_quit: false,
        resources: vec![
            ResourceSummary {
                name: String::from("Images"),
                count: 12,
                reclaimable_mb: 4200,
                // this means -> Images: 12 resources, 4200MB reclaimable
            },
            ResourceSummary {
                name: String::from("Containers"),
                count: 3,
                reclaimable_mb: 850,
            },
        ],
    };

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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

            // let content = Paragraph::new("Docker disk usage");
            let content = Paragraph::new(app.resource_text());

            let footer = Paragraph::new("Press q to quit");

            let paragraph = Paragraph::new("Docker disk usage").block(
                Block::default()
                    .title("Docker Cleaner")
                    .borders(Borders::ALL),
            );

            // frame.render_widget(paragraph, areas);
            frame.render_widget(header, areas[0]);
            frame.render_widget(content, areas[1]);
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
        if key == KeyCode::Char('q') {
            self.should_quit = true;
        }
    }

    fn resource_text(&self) -> String {
        self.resources
            .iter() // borrowing each item rather than consuming the vector
            .map(ResourceSummary::display_text) // transforming each
            // summary into text
            .collect::<Vec<_>>() //
            .join("\n")
    }
}

impl ResourceSummary {
    fn display_text(&self) -> String {
        format!(
            "{}: {} resources, {} MB reclaimable",
            self.name, self.count, self.reclaimable_mb
        )
    }
}
