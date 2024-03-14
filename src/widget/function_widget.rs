use crate::widget::{into_widget::IntoWidget, param::WidgetParam, Widget};

use crate::states::States;

use crate::WidgetFrame;
use crate::WidgetResult;

use std::cell::RefMut;
use std::marker::PhantomData;

use std::any::{Any, TypeId};
use std::collections::HashMap;

// A widget that comes from specifically a function.
pub struct FunctionWidget<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_for_func {
    ($($item:ident)*) => {
        impl<Func, $($item),*> Widget for FunctionWidget<($($item,)*), Func>
        where
                for<'a, 'b> &'a mut Func:
                    FnMut( $($item),* ) -> WidgetResult +
                    FnMut( $(<$item as WidgetParam>::Item<'b>),* ) -> WidgetResult,
            $($item: WidgetParam + 'static),*
        {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn call(&mut self, states: &mut States) -> WidgetResult {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<$($item),*>(
                    mut f: impl FnMut($($item),*) -> WidgetResult,
                    $($item: $item,)*
                ) -> WidgetResult {
                    f($($item),*)
                }

                $(
                    let $item = $item::retrieve(states);
                )*

                call_inner(&mut self.f, $($item),*)
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

macro_rules! impl_into_widget {
    ($($item:ident)*) => {
        impl<Func, $($item),*> IntoWidget<($($item,)*), ()> for Func
                where
                for<'a, 'b> &'a mut Func:
                    FnMut( $($item),* ) -> WidgetResult +
                    FnMut( $(<$item as WidgetParam>::Item<'b>),* ) -> WidgetResult,
            $($item: WidgetParam + 'static),*

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
