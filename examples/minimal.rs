use crossterm::event::KeyCode;
use ratatui::widgets::Paragraph;
use widgetui::*;

use std::{cell::RefMut, error::Error};

fn widget(frame: &mut WidgetFrame, mut events: RefMut<Events>) -> WidgetResult {
    frame.render_widget(Paragraph::new("Hello, world!"), frame.size());

    if events.key(KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?.widgets(widget).run()
}
