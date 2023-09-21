use crate::App;

/// This is a trait that will allow you to abstract the way you add widgets and states.
pub trait Set {
    fn register_set(&self, app: App) -> App;
}
