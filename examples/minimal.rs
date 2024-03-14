use crossterm::event::KeyCode;
use ratatui::widgets::Paragraph;
use widgetui::*;

use std::error::Error;

fn widget(mut frame: ResMut<WidgetFrame>, mut events: ResMut<Events>) -> WidgetResult {
    let size = frame.size();
    frame.render_widget(Paragraph::new("Hello, world!"), size);

    if events.key(KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?.widgets(widget).run()
}
