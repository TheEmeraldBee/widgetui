use std::{
    any::Any,
    error::Error,
    time::{Duration, SystemTime},
};

use ratatui::prelude::Backend;

use crate::{
    chunks::{ChunkBuilder, Chunks},
    events::Events,
    setup::{reset_terminal, restore_terminal, setup_terminal, Frame, Terminal},
    states::{States, Time},
    widget::Widget,
    WidgetResult,
};

/// The powerhouse of tui-helper, runs all defined widgets for you at a set framerate
pub struct App {
    terminal: Terminal,
    widgets: Vec<Widget>,
    chunk_builder: ChunkBuilder,
    states: States,
    clock: Duration,
}

impl App {
    pub fn new(
        chunk_builder: impl FnMut(&mut Frame, &mut States) -> WidgetResult + 'static,
        clock: u64,
    ) -> Result<Self, Box<dyn Error>> {
        let terminal = setup_terminal()?;
        Ok(Self {
            terminal,
            widgets: vec![],
            chunk_builder: Box::new(chunk_builder),
            states: States::default(),
            clock: Duration::from_millis(clock),
        }
        .register_state(Events::default()))
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
    pub fn register_widget(
        mut self,
        widget: impl FnMut(&mut Frame, &mut States) -> WidgetResult + 'static,
    ) -> Self {
        self.widgets.push(Box::new(widget));
        self
    }

    /// Add a stat to the system
    pub fn register_state<S: Any>(mut self, state: S) -> Self {
        self.states.register(state);
        self
    }

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

            self.states.get::<Chunks>()?.clear();
            self.chunk_builder.as_mut()(&mut frame, &mut self.states)?;

            self.states.get::<Events>()?.events.clear();

            let time = SystemTime::now();

            if crossterm::event::poll(self.clock)? {
                self.states
                    .get::<Events>()?
                    .events
                    .push(crossterm::event::read()?);
            }

            let total_time = SystemTime::now().duration_since(time)?;

            self.states.get::<Time>()?.set_duration(total_time);

            for widget in &mut self.widgets {
                widget(&mut frame, &mut self.states)?;
            }

            // Render Frame
            self.terminal.flush()?;

            self.terminal.swap_buffers();

            self.terminal.backend_mut().flush()?;

            // Handle App Events
            if self.states.get::<Events>()?.exit {
                return Ok(());
            }
        }
    }
}
