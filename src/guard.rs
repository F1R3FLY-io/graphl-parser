use std::ops::{Deref, DerefMut};

use crate::bindings;

#[allow(clippy::missing_safety_doc)]
pub(crate) unsafe trait Releasable {
    fn release(&mut self);
}

pub(crate) trait Guarded: Sized
where
    Self: Releasable,
{
    fn guarded(self) -> Guard<Self>;
}

impl<T> Guarded for T
where
    Self: Releasable,
{
    fn guarded(self) -> Guard<Self> {
        Guard { value: self }
    }
}

pub(crate) struct Guard<T>
where
    T: Releasable,
{
    value: T,
}

impl<T: Releasable> Deref for Guard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Releasable> DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for Guard<T>
where
    T: Releasable,
{
    fn drop(&mut self) {
        self.value.release();
    }
}

unsafe impl Releasable for bindings::Binding {
    fn release(&mut self) {
        unsafe { bindings::free_Binding(*self) }
    }
}

unsafe impl Releasable for bindings::GraphBinding {
    fn release(&mut self) {
        unsafe { bindings::free_GraphBinding(*self) }
    }
}

unsafe impl Releasable for bindings::Vertex {
    fn release(&mut self) {
        unsafe { bindings::free_Vertex(*self) }
    }
}

unsafe impl Releasable for bindings::Name {
    fn release(&mut self) {
        unsafe { bindings::free_Name(*self) }
    }
}

unsafe impl Releasable for bindings::Graph {
    fn release(&mut self) {
        unsafe { bindings::free_Graph(*self) }
    }
}

unsafe impl Releasable for bindings::LVar {
    fn release(&mut self) {
        unsafe { bindings::free_LVar(*self) }
    }
}

pub(crate) trait ResourceConsumer: Sized {
    type Target;

    fn consume<T>(self, f: impl FnOnce(Self::Target) -> *mut T) -> Option<Guard<*mut T>>
    where
        *mut T: Releasable;
}

macro_rules! impl_resource_consumer {
    ($($ty:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($ty),+> ResourceConsumer for ($(Guard<*mut $ty>,)+)
        where
            $(*mut $ty: Releasable,)+
        {
            type Target = ($(*mut $ty,)+);

            fn consume<T>(self, f: impl FnOnce(Self::Target) -> *mut T) -> Option<Guard<*mut T>>
            where
                *mut T: Releasable,
            {
                let ($($ty,)+) = self;

                let result = f(($(*$ty,)+));

                if result.is_null() {
                    None
                } else {
                    $(std::mem::forget($ty);)+
                    Some(result.guarded())
                }
            }
        }
    };
}

impl_resource_consumer!(R1);
impl_resource_consumer!(R1, R2);
impl_resource_consumer!(R1, R2, R3);
