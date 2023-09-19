extern crate tui_helper;

use std::{error::Error, time::Duration};

use ratatui::prelude::{Constraint, Direction, Layout};
use tui_helper::{
    widgets::message::{message, Message, MessageChunk},
    *,
};

fn chunk_builder(frame: &mut Frame, states: &mut States) -> WidgetResult {
    let chunks = states.get::<Chunks>()?;

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

fn my_widget(_frame: &mut Frame, states: &mut States) -> WidgetResult {
    if states
        .get::<Events>()?
        .key(crossterm::event::KeyCode::Char('m'))
    {
        states
            .get::<Message>()?
            .render_message("Custom Message", Duration::from_millis(500));
    }

    if states
        .get::<Events>()?
        .key(crossterm::event::KeyCode::Char('n'))
    {
        states
            .get::<Message>()?
            .render_message("Error", Duration::from_millis(50))
    }

    if states
        .get::<Events>()?
        .key(crossterm::event::KeyCode::Char('q'))
    {
        states.get::<Events>()?.register_exit();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    App::new(chunk_builder, 100)?
        .handle_panics()
        .register_widget(message)
        .register_widget(my_widget)
        .register_state(Message::default())
        .run()
}
