//! Concurrency primitives for the Particle photon

#![deny(missing_docs)]
#![feature(const_fn)]
#![no_std]

extern crate static_ref;

use core::cell::UnsafeCell;

use static_ref::{Ref, RefMut};

/// Application context
pub struct App {
    _0: (),
}

extern "C" {
    fn system_delay_ms(ms: u32, force_no_background_loop: bool);
}

impl App {
    /// Waits for `ms` milliseconds
    ///
    /// During this time, the scheduler may execute cloud functions so no
    /// outstanding borrows to global resources may exist when this method is
    /// called
    pub fn delay_ms(&mut self, ms: u32) {
        unsafe { system_delay_ms(ms, false) }
    }
}

/// Cloud function context
pub struct Cloud {
    _0: (),
}

/// A global resource that can be shared between different contexts
pub struct Resource<T> {
    data: UnsafeCell<T>,
}

impl<T> Resource<T> {
    /// Creates a new `Resource` with some initial `value`
    pub const fn new(value: T) -> Self {
        Resource { data: UnsafeCell::new(value) }
    }

    /// Grants immutable access to the resource
    pub fn access<'ctxt, C>(&'static self, _ctxt: &'ctxt C) -> Ref<'ctxt, T>
    where
        C: Ctxt,
    {
        unsafe { Ref::new(&*self.data.get()) }
    }

    /// Grants mutable access to the resource
    pub fn access_mut<'ctxt, C>(
        &'static self,
        _ctxt: &'ctxt mut C,
    ) -> RefMut<'ctxt, T>
    where
        C: Ctxt,
    {
        unsafe { RefMut::new(&mut *self.data.get()) }
    }
}

unsafe impl<T> Sync for Resource<T>
where
    T: Send,
{
}

/// Implementation detail. Do not implement this trait.
pub unsafe trait Ctxt {}

unsafe impl Ctxt for App {}
unsafe impl Ctxt for Cloud {}
