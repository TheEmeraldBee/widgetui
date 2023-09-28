use std::{
    any::Any,
    error::Error,
    time::{Duration, SystemTime},
};

use ratatui::prelude::Backend;

use crate::{
    chunks::{ChunkBuilder, Chunks},
    events::Events,
    set::Set,
    setup::{reset_terminal, restore_terminal, setup_terminal, WidgetFrame, WidgetTerminal},
    states::{States, Time},
    widgets::message::MessageState,
    IntoWidget, IntoWidgetSet, Widget, WidgetResult,
};

/// The powerhouse of tui-helper, runs all defined widgets for you at a set framerate
pub struct App {
    terminal: WidgetTerminal,
    widgets: Vec<Box<dyn Widget>>,
    states: States,
    clock: Duration,
}

impl App {
    /// Create a new app with the given clock time (in ms)
    pub fn new(clock: u64) -> Result<Self, Box<dyn Error>> {
        let terminal = setup_terminal()?;
        Ok(Self {
            terminal,
            widgets: vec![],
            states: States::default(),
            clock: Duration::from_millis(clock),
        }
        .state(Events::default()))
    }

    /// Running this will ensure that any panic that happens, this will catch
    /// And prevent your terminal from messing up.
    pub fn handle_panics(self) -> Self {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            reset_terminal().unwrap();
            original_hook(panic);
        }));

        self
    }

    /// Adds the following Widgets to the system.
    /// This will take in a tuple of widgets, or a single widget.
    pub fn widgets<I>(mut self, widget: impl IntoWidgetSet<I>) -> Self {
        for widget in widget.into_widget_set() {
            self.widgets.push(widget);
        }
        self
    }

    /// Add a state to the system
    pub fn state<S: Any>(mut self, state: S) -> Self {
        self.states.register(state);
        self
    }

    /// Add a set to the system
    pub fn set(self, set: impl Set) -> Self {
        set.register_set(self)
    }

    /// Run the app, returning an error if any of the functions error out.
    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        let result = self.inner_run();

        restore_terminal(self.terminal)?;

        result
    }

    fn inner_run(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.hide_cursor()?;

        loop {
            self.terminal.autoresize()?;
            let mut frame = self.terminal.get_frame();

            {
                let mut chunks = self.states.get::<Chunks>()?;
                let mut chunks = chunks.get();

                chunks.clear();

                let mut events = self.states.get::<Events>()?;
                let mut events = events.get();

                let mut time = self.states.get::<Time>()?;
                let mut time = time.get();

                events.event = None;

                let start_time = SystemTime::now();

                if crossterm::event::poll(self.clock)? {
                    events.event = Some(crossterm::event::read()?);
                }

                let total_time = SystemTime::now().duration_since(start_time)?;

                time.set_duration(total_time);
            }

            self.states.get::<Chunks>()?.get();

            for widget in &mut self.widgets {
                widget.call(&mut frame, &mut self.states)?;
            }

            // Render Frame
            self.terminal.flush()?;

            self.terminal.swap_buffers();

            self.terminal.backend_mut().flush()?;

            // Handle App Events
            if self.states.get::<Events>()?.get().exit {
                return Ok(());
            }
        }
    }
}
