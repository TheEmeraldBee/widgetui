use std::time::Duration;

use anyhow::Result;
use ratatui::prelude::{Constraint, Direction, Layout};
use widgetui::{
    widgets::message::{Message, MessageChunk, MessageState},
    *,
};

fn chunk_builder(frame: Res<'_, WidgetFrame>, mut chunks: ResMut<'_, Chunks>) -> WidgetResult {
    let popup = layout![
        frame.size(),
        (%50),
        (>3) => {
            %10,
            %80,
            %10
        },
        (%50)
    ][1][1];

    chunks.register_chunk::<MessageChunk>(popup);

    Ok(())
}

fn my_widget(mut events: ResMut<Events>, mut message: ResMut<MessageState>) -> WidgetResult {
    if events.key(crossterm::event::KeyCode::Char('m')) {
        message.render_message("Custom Message", Duration::from_millis(500));
    }

    if events.key(crossterm::event::KeyCode::Char('n')) {
        message.render_message("Cool", Duration::from_millis(500));
    }

    if events.key(crossterm::event::KeyCode::Char('q')) {
        events.register_exit()
    }

    Ok(())
}

fn main() -> Result<()> {
    App::new(100)?
        .handle_panics()
        .widgets((chunk_builder, my_widget))
        .sets(Message)
        .run()
}
