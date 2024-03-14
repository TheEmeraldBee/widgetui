use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::State;

/// A state that wraps over the events from crossterm
#[derive(Default, Clone, State)]
pub struct Events {
    pub event: Option<Event>,
    pub(crate) exit: bool,
}

impl Events {
    /// Returns whether a key was pressed this frame.
    pub fn key(&self, code: KeyCode) -> bool {
        if let Some(Event::Key(key_event)) = self.event {
            if key_event.code == code {
                return true;
            }
        }

        false
    }

    /// Returns whether a Key Event was completed this frame.
    pub fn key_event(&self, check_event: KeyEvent) -> bool {
        if let Some(Event::Key(key_event)) = self.event {
            if key_event == check_event {
                return true;
            }
        }

        false
    }

    /// Returns whether a key was pressed this frame.
    /// This will consume the key, not passing it on to future widgets.
    pub fn consume_key(&mut self, code: KeyCode) -> bool {
        if let Some(Event::Key(key_event)) = self.event {
            if key_event.code == code {
                self.event = None;
                return true;
            }
        }

        false
    }

    /// Let the app know you want to quit.
    pub fn register_exit(&mut self) {
        self.exit = true;
    }
}
