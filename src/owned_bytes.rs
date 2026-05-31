use core::ops::Deref;

use alloc::boxed::Box;

pub struct OwnedBytes {
    bytes: &'static [u8],
    dropper: Box<dyn Fn()>,
}

impl OwnedBytes {
    pub fn new<T: Deref<Target = [u8]> + 'static>(bytes: T) -> Self {
        let ptr = Box::into_raw(Box::new(bytes));
        let dropper = Box::new(move || {
            drop(unsafe { Box::<T>::from_raw(ptr) });
        });
        let contents = unsafe { &*ptr as &T };

        Self {
            bytes: contents,
            dropper,
        }
    }

    pub fn bytes(&self) -> &'static [u8] {
        self.bytes
    }
}

impl Deref for OwnedBytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

impl Drop for OwnedBytes {
    fn drop(&mut self) {
        (self.dropper)()
    }
}
