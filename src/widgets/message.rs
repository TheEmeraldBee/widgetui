use std::{collections::VecDeque, time::Duration};

use ratatui::widgets::{Block, Borders, Paragraph};

use crate::{states::Time, Chunks, Frame, States, WidgetResult};

pub struct MessageChunk;

pub struct Message {
    active_message: Option<(String, Duration)>,
    messages: VecDeque<(String, Duration)>,
    block: Block<'static>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            active_message: None,
            messages: VecDeque::default(),
            block: Block::new().title("Messages").borders(Borders::ALL),
        }
    }
}

impl Message {
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
pub fn message(frame: &mut Frame, states: &mut States) -> WidgetResult {
    let time = states.get::<Time>()?.clone().frame_time();
    let rect = states.get::<Chunks>()?.get_chunk::<MessageChunk>()?;

    if let Some(messages) = states.get_option::<Message>() {
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
    } else {
        return Err(anyhow!("Message widget expected a message state to exist").into());
    }

    Ok(())
}
