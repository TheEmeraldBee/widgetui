use std::marker::PhantomData;

use crate::widget::Widget;

/// Allows all versions of a function with the parameters
/// that work within a widget to be converted into a widget
pub trait IntoWidget<Input, Data> {
    type Widget: Widget;

    fn into_widget(self) -> Self::Widget;
}
