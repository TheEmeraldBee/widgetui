use std::error::Error;

use ratatui::widgets::Paragraph;
use widgetui::*;

#[derive(State)]
pub struct CustomState {
    state: i32,
}

pub struct CustomChunk;

pub fn handle_state(
    mut frame: ResMut<WidgetFrame>,
    mut custom_state: ResMut<CustomState>,
    mut events: ResMut<Events>,
    mut chunks: ResMut<Chunks>,
) -> WidgetResult {
    // Register A Test Chunk
    chunks.register_chunk::<CustomChunk>(frame.size());
    let chunk = chunks.get_chunk::<CustomChunk>()?;

    custom_state.state += 1;

    if custom_state.state >= 50 {
        events.register_exit();
    }

    frame.render_widget(
        Paragraph::new(format!("Custom State: {}", custom_state.state)),
        chunk,
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(App::new(100)?
        .states(CustomState { state: 0 })
        .widgets(handle_state)
        .handle_panics()
        .run()?)
}
