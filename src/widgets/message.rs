use std::{collections::VecDeque, time::Duration};

use ratatui::buffer::Buffer;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use crate::{states::Time, Chunks, WidgetFrame, WidgetResult};

pub struct MessageChunk;

use crate::{App, Res, ResMut, State};

#[derive(State)]
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
    mut frame: ResMut<Buffer>,
    timer: Res<Time>,
    chunks: Res<Chunks>,
    mut messages: ResMut<MessageState>,
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
            Paragraph::new(message.0.clone())
                .block(messages.block.clone())
                .render(rect, &mut frame)
        }
    } else if let Some(message) = messages.messages.pop_front() {
        messages.active_message = Some(message.clone());

        Paragraph::new(message.0)
            .block(messages.block.clone())
            .render(rect, &mut frame);
    }

    Ok(())
}

use crate::set::Set;

#[set]
pub fn Message(app: App) -> App {
    app.widgets(message).states(MessageState::default())
}
