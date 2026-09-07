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
}

fn main() -> io::Result<()> {
    let mut app = App { should_quit: false };

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
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

            let content = Paragraph::new("Docker disk usage");

            let footer = Paragraph::new("Press q to quit");

            let paragraph = Paragraph::new("Docker disk usage").block(
                Block::default()
                    .title("Docker Cleaner")
                    .borders(Borders::ALL),
            );
            // let block = Block::default()
            //     .title("Docker cleaner")
            //     .borders(Borders::ALL);

            // frame.render_widget(paragraph, areas);
            frame.render_widget(header, areas[0]);
            frame.render_widget(content, areas[1]);
            frame.render_widget(footer, areas[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
