use std::mem::ManuallyDrop;
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
        Guard {
            value: ManuallyDrop::new(self),
        }
    }
}

pub(crate) struct Guard<T>
where
    T: Releasable,
{
    value: ManuallyDrop<T>,
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

impl<T> Guard<T>
where
    T: Releasable,
{
    pub(crate) fn into_inner(self) -> T {
        let mut this = ManuallyDrop::new(self);
        unsafe { ManuallyDrop::take(&mut this.value) }
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
