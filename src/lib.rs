#![allow(unused_imports)]

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate tui_helper_proc_macro;

pub mod app;
pub mod chunks;
pub mod events;
pub mod layout;
pub mod set;
pub mod setup;
pub mod states;
pub mod widget;

/// Pre-Built Widgets
pub mod widgets;

pub use app::App;
pub use chunks::Chunks;
pub use events::Events;
pub use set::Set;
pub use setup::{WidgetBackend, WidgetFrame, WidgetTerminal};
pub use states::{MultiFromStates, State};
pub use widget::{into_widget_set::IntoWidgetSet, WidgetResult};

pub use tui_helper_proc_macro::set;
pub use tui_helper_proc_macro::State;

pub use widget::param::*;

// Re-Exports
pub use crossterm;
pub use ratatui;
