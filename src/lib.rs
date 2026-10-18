#![no_std]
//! [`Gltf`] is a runtime asset delivery format designed to bridge
//! the gap between 3D content creation tools and modern GL applications.
//!
//! # Versions
//! This implementation attempts to decode glTF files using the versions
//! [1.0](https://github.com/KhronosGroup/glTF/tree/main/specification/1.0)
//! and [2.0](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html)
//! of the specification.
//!
//! # Usage
//! Creating a [`Gltf`] can be used with any data structure that implements
//! [`Borrow<[u8>]`](core::borrow::Borrow).
//!
//! ```rust
//! # use can_i_haz_gltf::Gltf;
//! let data: &'static [u8] = b"{}";
//! let gltf = Gltf::new(data).unwrap();
//! ```
//!
//! ```rust
//! # use can_i_haz_gltf::Gltf;
//! let data: Vec<u8> = b"{}".to_vec();
//! let gltf = Gltf::new(data).unwrap();
//! ```
//!
//! ```rust
//! # use can_i_haz_gltf::Gltf;
//! let data: Box<[u8]> = (*b"{}").into();
//! let gltf = Gltf::new(data).unwrap();
//! ```
//!
//! ```rust
//! # extern crate alloc;
//! # use can_i_haz_gltf::Gltf;
//! # use alloc::rc::Rc;
//! let data: Rc<[u8]> = (*b"{}").into();
//! let gltf = Gltf::new(data).unwrap();
//! ```
//!
//! ```rust
//! # extern crate alloc;
//! # use can_i_haz_gltf::Gltf;
//! # use alloc::sync::Arc;
//! let data: Arc<[u8]> = (*b"{}").into();
//! let gltf = Gltf::new(data).unwrap();
//! ```
//!
//! ## Lifetimes
//! [`Gltf::new`] takes in a owned type bound by ['static], making it impossible to
//! create a [`Gltf`] from a reference.
//!
//! ```rust,compile_fail
//! # use can_i_haz_gltf::Gltf;
//! // Fails because `data` does not live long enough.
//! let data: Vec<u8> = b"{}".to_vec();
//! let gltf = Gltf::new(data.as_slice()).unwrap();
//! let _  = gltf.version();
//! ```
//!

extern crate alloc;

pub mod json;
pub mod owned_t;
pub mod str_cursor;
mod validation_error;

use alloc::borrow::Cow;

pub use validation_error::GltfValidationError;

const GLTF_MAGIC: [u8; 4] = [0x67, 0x6C, 0x54, 0x46];
const GLTF_JSON_CHUNK_TYPE: u32 = 0x4E4F534A;
const GLTF_BIN_CHUNK_TYPE: u32 = 0x004E4942;

pub struct Gltf<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> Gltf<'a> {
    pub fn new(data: Cow<'a, [u8]>) -> Result<Self, GltfValidationError<'a>> {
        // if contents[0] == b'{' {
        //     Self::gltf_new(contents)
        // } else if contents[..4] != GLTF_MAGIC {
        //     Err(GltfValidationError::WrongMagic)
        // } else {
        //     Self::glb_new(contents)
        // }
        unimplemented!()
    }
}
