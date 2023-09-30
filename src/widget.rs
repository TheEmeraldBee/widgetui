use std::{
    cell::RefMut,
    error::Error,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    chunks::Chunks,
    setup::WidgetFrame,
    states::{FromState, State, States},
    App,
};

pub type WidgetResult = Result<(), Box<dyn Error>>;

/// Trait that enables the type implemented to be created from a state.
pub trait FromStates {
    fn from_state(states: &mut States) -> Result<State<Self>, Box<dyn Error>>;
}

/// Trait that enables tuples to be registered into the app.
/// Does not need to be manually implemented
pub trait MultiFromStates {
    fn insert_states(self, app: App) -> App;
}

impl<T: FromStates + 'static> MultiFromStates for T {
    fn insert_states(self, mut app: App) -> App {
        app.states.register(self);
        app
    }
}

macro_rules! impl_for_state_tuple {
    ($($item:ident $num:tt)*) => {
        impl <$($item: FromStates + 'static),*> MultiFromStates for ($($item,)*) {
            fn insert_states(self, mut app: App) -> App {
                $(app.states.register(self.$num);)*
                app
            }
        }
    }
}

impl_for_state_tuple! { A0 0 }
impl_for_state_tuple! { A0 0 A1 1 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 A14 14 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 A14 14 A15 15 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 A14 14 A15 15 A16 16 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 A14 14 A15 15 A16 16 A17 17 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 A14 14 A15 15 A16 16 A17 17 A18 18 }
impl_for_state_tuple! { A0 0 A1 1 A2 2 A3 3 A4 4 A5 5 A6 6 A7 7 A8 8 A9 9 A10 10 A11 11 A12 12 A13 13 A14 14 A15 15 A16 16 A17 17 A18 18 A19 19 }

/// A widget that can be called.
pub trait Widget {
    fn call(&mut self, frame: &mut WidgetFrame, states: &mut States) -> WidgetResult;
}

/// A widget that comes from specifically a function.
pub struct FunctionWidget<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_for_func {
    ($($item:ident)*) => {
        impl<Func, $($item),*> Widget for FunctionWidget<($($item,)*), Func>
        where
            Func: FnMut(&mut WidgetFrame, $(RefMut<$item>),*) -> WidgetResult,
            $($item: FromStates),*
        {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn call(&mut self, frame: &mut WidgetFrame, states: &mut States) -> WidgetResult {
                $(let mut $item = $item::from_state(states)?;)*
                (self.f)(frame, $($item.get()),*)
            }
        }
    };
}

impl_for_func! {}
impl_for_func! { A }
impl_for_func! { A B }
impl_for_func! { A B C }
impl_for_func! { A B C D }
impl_for_func! { A B C D E }
impl_for_func! { A B C D E F }
impl_for_func! { A B C D E F G }
impl_for_func! { A B C D E F G H }
impl_for_func! { A B C D E F G H I }
impl_for_func! { A B C D E F G H I J }
impl_for_func! { A B C D E F G H I J K }
impl_for_func! { A B C D E F G H I J K L }

/// Allows all versions of a function with the parameters
/// that work within a widget to be converted into a widget
pub trait IntoWidget<Input> {
    type Widget: Widget;

    fn into_widget(self) -> Self::Widget;
}

macro_rules! impl_into_widget {
    ($($item:ident)*) => {
        impl<Func, $($item: FromStates),*> IntoWidget<($($item,)*)> for Func
        where Func: FnMut(&mut WidgetFrame, $(RefMut<$item>),*)
         -> WidgetResult
        {
            type Widget = FunctionWidget<($($item,)*), Self>;
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn into_widget(self) -> Self::Widget {
                FunctionWidget {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

impl_into_widget! {}
impl_into_widget! { A }
impl_into_widget! { A B }
impl_into_widget! { A B C }
impl_into_widget! { A B C D }
impl_into_widget! { A B C D E }
impl_into_widget! { A B C D E F }
impl_into_widget! { A B C D E F G }
impl_into_widget! { A B C D E F G H }
impl_into_widget! { A B C D E F G H I }
impl_into_widget! { A B C D E F G H I J }
impl_into_widget! { A B C D E F G H I J K }
impl_into_widget! { A B C D E F G H I J K L }

pub trait IntoWidgetSet<Inputs> {
    fn into_widget_set(self) -> Vec<Box<dyn Widget>>;
}

impl<I1, W: Widget + 'static, Func1> IntoWidgetSet<I1> for Func1
where
    Func1: IntoWidget<I1, Widget = W>,
{
    fn into_widget_set(self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(self.into_widget())]
    }
}

macro_rules! impl_into_widget_set {
    ($($input:ident $widget:ident $func:ident $num:tt)*) => {
        impl<$($input,)* $($widget: Widget + 'static,)* $($func),*> IntoWidgetSet<($($input,)*)> for ($($func,)*)
        where
            $($func: IntoWidget<$input, Widget = $widget>),*
        {
            fn into_widget_set(self) -> Vec<Box<dyn Widget>> {
                vec![
                    $(Box::new(self.$num.into_widget())),*
                ]
            }
        }
    };
}

impl_into_widget_set! {I0 W0 F0 0  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13 I14 W14 F14 14  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13 I14 W14 F14 14 I15 W15 F15 15  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13 I14 W14 F14 14 I15 W15 F15 15 I16 W16 F16 16  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13 I14 W14 F14 14 I15 W15 F15 15 I16 W16 F16 16 I17 W17 F17 17  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13 I14 W14 F14 14 I15 W15 F15 15 I16 W16 F16 16 I17 W17 F17 17 I18 W18 F18 18  }
impl_into_widget_set! {I0 W0 F0 0 I1 W1 F1 1 I2 W2 F2 2 I3 W3 F3 3 I4 W4 F4 4 I5 W5 F5 5 I6 W6 F6 6 I7 W7 F7 7 I8 W8 F8 8 I9 W9 F9 9 I10 W10 F10 10 I11 W11 F11 11 I12 W12 F12 12 I13 W13 F13 13 I14 W14 F14 14 I15 W15 F15 15 I16 W16 F16 16 I17 W17 F17 17 I18 W18 F18 18 I19 W19 F19 19  }
