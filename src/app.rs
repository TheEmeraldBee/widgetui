use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    error::Error,
    io,
    ops::Deref,
    time::{Duration, SystemTime},
};

use ratatui::{buffer::Buffer, prelude::Backend};

use crate::{
    chunks::Chunks,
    events::Events,
    set::{Set, Sets},
    setup::{reset_terminal, restore_terminal, setup_terminal, WidgetFrame, WidgetTerminal},
    states::{MultiFromStates, States, Time},
    widget::{into_widget::IntoWidget, into_widget_set::IntoWidgetSet, Widget},
    widgets::message::MessageState,
    Res, ResMut, WidgetParam,
};

/// The powerhouse of widgetui, runs all defined widgets for you
pub struct App {
    terminal: WidgetTerminal,
    widgets: Vec<Box<dyn Widget>>,
    pub(crate) states: States,
    clock: Duration,
}

impl App {
    /// Create a new app with the given clock time (in ms)
    pub fn new(clock: u64) -> Result<Self, io::Error> {
        let terminal = setup_terminal()?;

        Ok(Self {
            terminal,
            widgets: vec![],
            states: HashMap::new(),
            clock: Duration::from_millis(clock),
        }
        .handle_panics()
        .states((Chunks::default(), Time::default(), Events::default())))
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
    pub fn widgets<I, T>(mut self, widget: impl IntoWidgetSet<I, T>) -> Self {
        for widget in widget.into_widget_set() {
            self.widgets.push(widget);
        }
        self
    }

    pub fn widget<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.widgets.push(Box::new(widget));
        self
    }

    /// Add the following states to the system
    /// This will take in a state or a tuple of states.
    pub fn states<S: MultiFromStates>(self, state: S) -> Self {
        state.insert_states(self)
    }

    /// Add a set to the system
    pub fn sets(self, set: impl Sets) -> Self {
        set.register_sets(self)
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

            let widget_frame = WidgetFrame {
                cursor_position: None,
                buffer: frame.buffer_mut().clone(),
                viewport_area: frame.size(),
                count: frame.count(),
            };

            self.states.insert(
                TypeId::of::<WidgetFrame>(),
                RefCell::new(Box::new(widget_frame)),
            );

            {
                let mut chunks = ResMut::<Chunks>::retrieve(&self.states);

                chunks.clear();

                let mut events = ResMut::<Events>::retrieve(&self.states);

                let mut time = ResMut::<Time>::retrieve(&self.states);

                events.event = None;

                let start_time = SystemTime::now();

                if crossterm::event::poll(self.clock)? {
                    events.event = Some(crossterm::event::read()?);
                }

                let total_time = SystemTime::now().duration_since(start_time)?;

                time.set_duration(total_time);
            }

            for widget in &mut self.widgets {
                widget.call(&mut self.states)?;
            }

            // Update the window.
            {
                let widget_frame = Res::<WidgetFrame>::retrieve(&self.states);

                if let Some((x, y)) = widget_frame.cursor_position {
                    frame.set_cursor(x, y);
                }

                *frame.buffer_mut() = widget_frame.buffer.clone();
            }

            // Render Frame
            self.terminal.flush()?;

            self.terminal.swap_buffers();

            self.terminal.backend_mut().flush()?;

            // Handle App Events
            if ResMut::<Events>::retrieve(&self.states).exit {
                return Ok(());
            }
        }
    }
}
