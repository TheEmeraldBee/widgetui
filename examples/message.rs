use std::{cell::RefMut, error::Error, time::Duration};

use ratatui::prelude::{Constraint, Direction, Layout};
use widgetui::{
    widgets::message::{Message, MessageChunk, MessageState},
    *,
};

fn chunk_builder(frame: &mut Frame, mut chunks: RefMut<Chunks>) -> WidgetResult {
    let popup = layout![
        frame.size(),
        constraint!(%50),
        constraint!(>3) => {
            constraint!(%10),
            constraint!(%80),
            constraint!(%10)
        },
        constraint!(%50)
    ][1][1];

    chunks.register_chunk::<MessageChunk>(popup);

    Ok(())
}

fn my_widget(
    _frame: &mut Frame,
    mut events: RefMut<Events>,
    mut message: RefMut<MessageState>,
) -> WidgetResult {
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

fn main() -> Result<(), Box<dyn Error>> {
    App::new(100)?
        .handle_panics()
        .with_widget(chunk_builder)
        .with_widget(my_widget)
        .with_set(Message)
        .run()
}
