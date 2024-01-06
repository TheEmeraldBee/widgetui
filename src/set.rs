use crate::App;

/// This is a trait that will allow you to abstract the way you add widgets and states.
pub trait Set {
    fn register_set(&self, app: App) -> App;
}

pub trait Sets {
    fn register_sets(&self, app: App) -> App;
}

impl<A> Sets for A
where
    A: Set,
{
    fn register_sets(&self, app: App) -> App {
        self.register_set(app)
    }
}

impl<A> Sets for (A,)
where
    A: Set,
{
    fn register_sets(&self, app: App) -> App {
        self.0.register_set(app)
    }
}

macro_rules! impl_sets {
    ($($t:ident $val:tt)*) => {
        impl<$($t,)*> Sets for ($($t,)*) where $($t: Set,)* {
            fn register_sets(&self, mut app: App) -> App {
                $(app = self.$val.register_set(app);)*
                app
            }
        }
    };
}

impl_sets! {A 0 B 1}
impl_sets! {A 0 B 1 C 2 }
impl_sets! {A 0 B 1 C 2 D 3 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 F 5 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 F 5 G 6 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 }
impl_sets! {A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 0}
