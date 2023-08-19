mod app;
use app::app::Notes;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn run_app(mut notes: Notes) {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(_) => panic!("Unable to create terminal"),
    };
    terminal
        .draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })
        .unwrap();
    loop {}
}

fn main() -> Result<(), io::Error> {
    let notes: Notes = app::app::get_saved_data("data.txt");
    run_app(notes);
    Ok(())
}
