use core::ops::Deref;

use alloc::boxed::Box;

pub struct OwnedT<T: 'static + ?Sized> {
    data: &'static T,
    dropper: Box<dyn Fn()>,
}

pub type OwnedBytes = OwnedT<[u8]>;

impl<T: 'static + ?Sized> OwnedT<T> {
    pub fn new<D: Deref<Target = T> + 'static>(data: D) -> Self {
        let ptr = Box::into_raw(Box::new(data));
        let dropper = Box::new(move || {
            drop(unsafe { Box::<D>::from_raw(ptr) });
        });
        let contents = unsafe { &*ptr as &D };

        Self {
            data: contents,
            dropper,
        }
    }

    pub fn data(&self) -> &'static T {
        self.data
    }
}

impl<T: 'static + ?Sized> Deref for OwnedT<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T: 'static + ?Sized> Drop for OwnedT<T> {
    fn drop(&mut self) {
        (self.dropper)()
    }
}
