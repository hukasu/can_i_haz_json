use core::ops::Deref;

use alloc::boxed::Box;

/// Wraps an owned object that [`Deref`]s into `T` in an object that
/// can produce `'static` references of `T`.
///
/// # Safety
/// This might introduce UBs as the lifetime is unsafely
/// downcast from `'1` to `'static`.
///
/// ```rust
/// # use can_i_haz_gltf::owned_t::OwnedBytes;
/// let static_ref: &'static [u8] = {
///     let owned_bytes = OwnedBytes::new(vec![48u8, 49, 50, 51]);
///     // This is an UB as the pointer becames dangling
///     unsafe { owned_bytes.data() }
/// };
/// ```
///
/// Dereferencing returns a value with the correct lifetime.
/// ```rust,compile_fail
/// # use can_i_haz_gltf::owned_t::OwnedBytes;
/// let static_ref: &'static [u8] = {
///     let owned_bytes = OwnedBytes::new(vec![48u8, 49, 50, 51]);
///     // fails with E0597
///     &*owned_bytes
/// };
/// ```
pub struct OwnedT<T: 'static + ?Sized> {
    data: &'static T,
    dropper: Box<dyn Fn()>,
}

pub type OwnedBytes = OwnedT<[u8]>;
pub type OwnedStr = OwnedT<str>;

impl<T: 'static + ?Sized> OwnedT<T> {
    /// Dereferences the data and wraps it into an object that can produce
    /// `'static` references of that data.
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

    /// Returns a reference to the dereferenced data.
    ///
    /// # Safety
    /// The `'static` lifetime is downcast from `'1`, and so
    /// might became dangling.
    pub unsafe fn data(&self) -> &'static T {
        self.data
    }
}

impl<T: 'static + ?Sized> Deref for OwnedT<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T: 'static + ?Sized + PartialEq> PartialEq for OwnedT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: 'static + ?Sized + Eq> Eq for OwnedT<T> {}

impl<T: 'static + ?Sized> Drop for OwnedT<T> {
    fn drop(&mut self) {
        (self.dropper)()
    }
}
