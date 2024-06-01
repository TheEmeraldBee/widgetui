pub mod param;

pub mod function_widget;
pub mod into_widget;
pub mod into_widget_set;

use crate::{states::States, WidgetFrame};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WidgetError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Chunk doesn't exist")]
    ChunkError,
    #[error(transparent)]
    Misc(#[from] anyhow::Error),
}

/// The main result that a widget will always return.
pub type WidgetResult = Result<(), WidgetError>;

/// A widget that can be called.
pub trait Widget {
    fn call(&mut self, states: &mut States) -> WidgetResult;
}
