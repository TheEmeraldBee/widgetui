use std::error::Error;

use ratatui::{
    prelude::{Constraint, Direction, Layout},
    widgets::Paragraph,
};
use widgetui::*;

struct TestChunk;

pub fn chunk_generator(frame: Res<WidgetFrame>, mut chunks: ResMut<Chunks>) -> WidgetResult {
    // A Custom macro to simplify creating your chunks!
    let chunk = layout! {
        frame.size(),
        (%50),
        (#1) => {#3, %100, #3},
        (%50)
    }[1][1];

    chunks.register_chunk::<TestChunk>(chunk);

    Ok(())
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    mut events: ResMut<Events>,
) -> WidgetResult {
    let chunk = chunks.get_chunk::<TestChunk>()?;

    frame.render_widget(Paragraph::new("Hello, world!"), chunk);

    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?
        .handle_panics()
        .widgets((chunk_generator, render))
        .run()
}
