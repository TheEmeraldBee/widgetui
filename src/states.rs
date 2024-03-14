use std::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    error::Error,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};

use crate::App;

pub type States = HashMap<TypeId, RefCell<Box<dyn Any>>>;

pub trait State: Any {}

/// Trait that enables tuples to be registered into the app.
/// Does not need to be manually implemented
pub trait MultiFromStates {
    fn insert_states(self, app: App) -> App;
}

impl<T: State + 'static> MultiFromStates for T {
    fn insert_states(self, mut app: App) -> App {
        app.states
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(self)));
        app
    }
}

macro_rules! impl_multi_from_states {
    ($($item:ident $num:tt)*) => {
        impl<$($item: State + 'static),*> MultiFromStates for ($($item,)*) {
            fn insert_states(self, mut app: App) -> App {
                $(app.states.insert(TypeId::of::<$item>(), RefCell::new(Box::new(self.$num)));)*
                app
            }
        }
    }
}

impl_multi_from_states! { A 0 }
impl_multi_from_states! { A 0 B 1 }
impl_multi_from_states! { A 0 B 1 C 2 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 E 4 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 E 4 F 5 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 }
impl_multi_from_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9}

// ---------- Guarenteed States --------- //

/// The state that will store anything time related for the system.
#[derive(Default, Clone, State)]
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
