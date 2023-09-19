use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{
    chunks::Chunks,
    setup::Frame,
    states::{FromState, States},
};

pub type Widget = Box<dyn FnMut(&mut Frame, &mut States) -> WidgetResult>;

pub type WidgetResult = Result<(), Box<dyn Error>>;
