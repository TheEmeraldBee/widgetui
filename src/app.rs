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
    setup::{reset_terminal, restore_terminal, setup_terminal, Frame, Terminal},
    states::{States, Time},
    widgets::message::MessageState,
    IntoWidget, Widget, WidgetResult,
};

/// The powerhouse of tui-helper, runs all defined widgets for you at a set framerate
pub struct App {
    terminal: Terminal,
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
        .with_state(Events::default()))
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

    /// Add a widget to the system
    pub fn with_widget<I, W: Widget + 'static>(
        mut self,
        widget: impl IntoWidget<I, Widget = W> + 'static,
    ) -> Self {
        self.widgets.push(Box::new(widget.into_widget()));
        self
    }

    /// Add a state to the system
    pub fn with_state<S: Any>(mut self, state: S) -> Self {
        self.states.register(state);
        self
    }

    /// Add a set to the system
    pub fn with_set(self, set: impl Set) -> Self {
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

                events.events.clear();

                let start_time = SystemTime::now();

                if crossterm::event::poll(self.clock)? {
                    events.events.push(crossterm::event::read()?);
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
