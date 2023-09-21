use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::FromStates;
use crate::State;
use crate::States;

/// A state that wraps over the events from crossterm
#[derive(Default, Clone, FromState)]
pub struct Events {
    pub events: Vec<Event>,
    pub(crate) exit: bool,
}

impl Events {
    /// Returns whether a key was pressed this frame.
    pub fn key(&self, code: KeyCode) -> bool {
        for event in &self.events {
            if let Event::Key(key_event) = event {
                if key_event.code == code {
                    return true;
                }
            }
        }

        false
    }

    /// Returns whether a Key Event was completed this frame.
    pub fn key_event(&self, check_event: KeyEvent) -> bool {
        for event in &self.events {
            if let Event::Key(key_event) = event {
                if *key_event == check_event {
                    return true;
                }
            }
        }

        false
    }

    /// Returns whether a key was pressed this frame.
    /// This will consume the key, not passing it on to future widgets.
    pub fn consume_key(&mut self, code: KeyCode) -> bool {
        let mut contained = false;
        for event in &self.events {
            if let Event::Key(key_event) = event {
                if key_event.code == code {
                    contained = true;
                    break;
                }
            }
        }

        self.events.retain(|event| {
            if let Event::Key(key_event) = event {
                if key_event.code == code {
                    return false;
                }
            }
            true
        });

        contained
    }

    /// Let the app know you want to quit.
    pub fn register_exit(&mut self) {
        self.exit = true;
    }
}
