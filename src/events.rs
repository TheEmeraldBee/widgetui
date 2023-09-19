use crossterm::event::{Event, KeyCode, KeyEvent};

#[derive(Default, Clone)]
pub struct Events {
    pub events: Vec<Event>,
    pub(crate) exit: bool,
}

impl Events {
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

    pub fn register_exit(&mut self) {
        self.exit = true;
    }
}
