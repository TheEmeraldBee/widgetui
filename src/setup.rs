use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;

pub type Backend = CrosstermBackend<Stdout>;
pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
pub type Terminal = ratatui::Terminal<Backend>;

pub fn setup_terminal() -> Result<Terminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

/// Resets the terminal.
pub fn reset_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
