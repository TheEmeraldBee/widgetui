use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::widgets::Paragraph;
use widgetui::*;

fn widget(mut frame: ResMut<WidgetFrame>, mut events: ResMut<Events>) -> WidgetResult {
    let size = frame.size();
    frame.render_widget(Paragraph::new("Hello, world!"), size);

    if events.key(KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<()> {
    App::new(100)?.widgets(widget).run()
}
