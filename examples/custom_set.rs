use std::{cell::RefMut, error::Error};

use ratatui::widgets::Paragraph;
use widgetui::*;

// This creates a state! No derive macro required
pub struct CoolState {
    pub q_count: i32,
}

impl Default for CoolState {
    fn default() -> Self {
        Self { q_count: 8 }
    }
}

pub fn widget(
    frame: &mut WidgetFrame,
    mut events: RefMut<Events>,
    mut state: RefMut<CoolState>,
) -> WidgetResult {
    if events.key(crossterm::event::KeyCode::Char('q')) {
        state.q_count -= 1;
        if state.q_count <= 0 {
            events.register_exit();
            return Ok(());
        }
    }

    frame.render_widget(
        Paragraph::new(format!("Press `q` {} more times", state.q_count)),
        frame.size(),
    );

    Ok(())
}

#[set]
pub fn CoolSet(app: App) -> App {
    app.widgets(widget).states(CoolState::default())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?.sets(CoolSet).run()
}
