extern crate tui_helper;

use tui_helper::*;

use std::{cell::RefMut, error::Error};

fn widget(_frame: &mut Frame, mut events: RefMut<Events>) -> WidgetResult {
    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?.with_widget(widget).run()
}
