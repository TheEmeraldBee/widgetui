use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
};

use ratatui::prelude::Rect;

use crate::{setup::WidgetFrame, States, WidgetResult};

use crate::states::State;
use crate::FromStates;

#[derive(Default, FromState)]
pub struct Chunks {
    chunks: HashMap<TypeId, Rect>,
}

impl Chunks {
    pub fn clear(&mut self) {
        self.chunks = HashMap::new();
    }

    pub fn register_chunk<T: Any>(&mut self, rect: Rect) {
        self.chunks.insert(TypeId::of::<T>(), rect);
    }

    pub fn get_chunk<T: Any>(&self) -> Result<Rect, Box<dyn Error>> {
        return match self.chunks.get(&TypeId::of::<T>()).cloned() {
            Some(chunk) => Ok(chunk),
            None => Err(anyhow!("Chunk doesn't exist").into()),
        };
    }
}
