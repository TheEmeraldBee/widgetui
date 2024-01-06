use std::cell::RefMut;
use std::{collections::VecDeque, time::Duration};

use ratatui::widgets::{Block, Borders, Paragraph};

use crate::{states::Time, Chunks, States, WidgetFrame, WidgetResult};

pub struct MessageChunk;

use crate::states::State;
use crate::{App, FromStates};

pub struct MessageState {
    active_message: Option<(String, Duration)>,
    messages: VecDeque<(String, Duration)>,
    block: Block<'static>,
}

impl Default for MessageState {
    fn default() -> Self {
        Self {
            active_message: None,
            messages: VecDeque::default(),
            block: Block::new().title("Messages").borders(Borders::ALL),
        }
    }
}

impl MessageState {
    pub fn new_custom_block(block: Block<'static>) -> Self {
        Self {
            active_message: None,
            messages: VecDeque::default(),
            block,
        }
    }

    pub fn render_message(&mut self, message: impl Into<String>, duration: Duration) {
        self.messages.push_back((message.into(), duration))
    }
}

/// A Timed Message Render
pub fn message(
    frame: &mut WidgetFrame,
    timer: RefMut<Time>,
    chunks: RefMut<Chunks>,
    mut messages: RefMut<MessageState>,
) -> WidgetResult {
    let time = timer.frame_time();
    let rect = chunks.get_chunk::<MessageChunk>()?;

    if let Some(message) = &mut messages.active_message {
        if message.1.as_millis() > time.as_millis() {
            message.1 -= time;
        } else {
            message.1 = Duration::ZERO;
        }

        if message.1.is_zero() {
            messages.active_message = None;
        } else {
            frame.render_widget(
                Paragraph::new(message.0.clone()).block(messages.block.clone()),
                rect,
            )
        }
    } else if let Some(message) = messages.messages.pop_front() {
        messages.active_message = Some(message.clone());

        frame.render_widget(
            Paragraph::new(message.0).block(messages.block.clone()),
            rect,
        );
    }

    Ok(())
}

use crate::set::Set;

#[set]
pub fn Message(app: App) -> App {
    app.widgets(message).states(MessageState::default())
}
