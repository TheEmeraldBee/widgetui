use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;

pub type WidgetBackend = CrosstermBackend<Stdout>;
pub type WidgetFrame<'a> = ratatui::Frame<'a>;
pub type WidgetTerminal = ratatui::Terminal<WidgetBackend>;

/// Sets up the terminal to work with your app
/// This is run automatically by app.
pub fn setup_terminal() -> Result<WidgetTerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = WidgetTerminal::new(backend)?;
    Ok(terminal)
}

/// Takes down the terminal, ensuring that it is all ok.
/// This is run automatically by the app.
pub fn restore_terminal(mut terminal: WidgetTerminal) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

/// Resets the terminal in case of a panic.
/// This is handled automatically if panic handler is enabled.
pub fn reset_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
