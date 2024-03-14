pub mod param;

pub mod function_widget;
pub mod into_widget;
pub mod into_widget_set;

use crate::{states::States, WidgetFrame};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
};

/// The main result that a widget will always return.
pub type WidgetResult = Result<(), Box<dyn Error>>;

/// A widget that can be called.
pub trait Widget {
    fn call(&mut self, states: &mut States) -> WidgetResult;
}
