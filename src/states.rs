use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
    collections::HashMap,
    error::Error,
    rc::Rc,
    time::Duration,
};

use crate::{Chunks, Events};

pub struct States {
    states: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for States {
    fn default() -> Self {
        let mut states = Self {
            states: HashMap::new(),
        };

        states.register(Time::default());
        states.register(Events::default());
        states.register(Chunks::default());

        states
    }
}

impl States {
    pub fn register<S: Any>(&mut self, state: S) {
        self.states.insert(state.type_id(), Box::new(state));
    }

    pub fn get_option<S: Any>(&mut self) -> Option<&mut S> {
        if let Some(state) = self.states.get_mut(&TypeId::of::<S>()) {
            state.downcast_mut::<S>()
        } else {
            None
        }
    }

    pub fn get<S: Any>(&mut self) -> Result<&mut S, Box<dyn Error>> {
        match self.get_option::<S>() {
            Some(item) => Ok(item),
            None => Err(anyhow!("Item didn't exist").into()),
        }
    }
}

pub trait FromState {
    fn from_state(states: &mut States) -> &mut Self;
}

// ---------- Guarenteed States --------- //

#[derive(Default, Clone)]
pub struct Time {
    frame_duration: Duration,
}

impl Time {
    pub fn set_duration(&mut self, duration: Duration) {
        self.frame_duration = duration
    }

    pub fn frame_time(&self) -> Duration {
        self.frame_duration
    }
}
