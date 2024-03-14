use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::states::States;

pub trait WidgetParam {
    type Item<'new>;
    fn retrieve(resources: &States) -> Self::Item<'_>;
}

pub struct Res<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'static> Deref for Res<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value.downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> WidgetParam for Res<'a, T> {
    type Item<'new> = Res<'new, T>;
    fn retrieve(resources: &States) -> Self::Item<'_> {
        Res {
            value: resources.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}

pub struct ResMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}

impl<'a, T: 'static> WidgetParam for ResMut<'a, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve(resources: &States) -> Self::Item<'_> {
        ResMut {
            value: resources
                .get(&TypeId::of::<T>())
                .unwrap_or_else(|| {
                    panic!(
                        "Resource: `{}` with id `{:?}` Not Found",
                        type_name::<T>(),
                        TypeId::of::<T>()
                    )
                })
                .borrow_mut(),
            _marker: PhantomData,
        }
    }
}
