use std::{
    cell::RefMut,
    error::Error,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    chunks::Chunks,
    setup::Frame,
    states::{FromState, State, States},
};

pub type WidgetResult = Result<(), Box<dyn Error>>;

/// Trait that enables the type implemented to be created from a state.
pub trait FromStates {
    fn from_state(states: &mut States) -> Result<State<Self>, Box<dyn Error>>;
}

/// A widget that can be called.
pub trait Widget {
    fn call(&mut self, frame: &mut Frame, states: &mut States) -> WidgetResult;
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
            Func: FnMut(&mut Frame, $(RefMut<$item>),*) -> WidgetResult,
            $($item: FromStates),*
        {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn call(&mut self, frame: &mut Frame, states: &mut States) -> WidgetResult {
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

pub trait IntoWidget<Input> {
    type Widget: Widget;

    fn into_widget(self) -> Self::Widget;
}

macro_rules! impl_into_widget {
    ($($item:ident)*) => {
        impl<Func, $($item: FromStates),*> IntoWidget<($($item,)*)> for Func
        where Func: FnMut(&mut Frame, $(RefMut<$item>),*)
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
