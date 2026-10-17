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

use core::fmt::Display;

use alloc::borrow::Cow;

use crate::json::JsonParsingError;

const GLTF_MAGIC: [u8; 4] = [0x67, 0x6C, 0x54, 0x46];
const GLTF_JSON_CHUNK_TYPE: u32 = 0x4E4F534A;
const GLTF_BIN_CHUNK_TYPE: u32 = 0x004E4942;

pub struct Gltf<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> Gltf<'a> {
    pub fn new(data: Cow<'a, [u8]>) -> Result<Self, GltfValidationError> {
        // if contents[0] == b'{' {
        //     Self::gltf_new(contents)
        // } else if contents[..4] != GLTF_MAGIC {
        //     Err(GltfValidationError::WrongMagic)
        // } else {
        //     Self::glb_new(contents)
        // }
        Ok(Gltf { data })
    }
}

/// A [`Gltf`] failed validation during creating
#[derive(Debug)]
pub enum GltfValidationError {
    /// The [`Gltf`] had the wrong magic.
    WrongMagic,
    /// The [`Gltf`] has a length that can't be addressed
    /// on the current system
    Unaddressable,
    /// The first chunk of the [`Gltf`] was not a JSON chunk
    MissingJsonChunk,
    /// A chunk in the [`Gltf`] had an unknown chunk type
    UnknownChunk { chunk: u32, chunk_type: u32 },
    /// [`Gltf`] had an unknown version
    UnknownVersion,
    /// [`Gltf`] using the 1.0 must have the binary chunk start aligned to
    /// 4 bytes.
    UnalignedBinaryChunk,
    /// An error occurred while decoding [`Gltf`] json chunk.
    Json(JsonParsingError),
}

impl Display for GltfValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongMagic => {
                write!(f, "Binary glTF had wrong magic header value.")
            }
            Self::Unaddressable => {
                write!(f, "GlTF file can't be addressed on the current system.")
            }
            Self::MissingJsonChunk => {
                write!(f, "GlTF file did not have a JSON chunk.")
            }
            Self::UnknownChunk { chunk, chunk_type } => {
                write!(
                    f,
                    "Chunk {} of the glTF file had the unknown chunk type of {:#08x}. Must be {:#08x} for JSON or {:#08x} for binary.",
                    chunk, chunk_type, GLTF_JSON_CHUNK_TYPE, GLTF_BIN_CHUNK_TYPE
                )
            }
            Self::UnknownVersion => {
                write!(f, "GlTF had unknown version.")
            }
            Self::UnalignedBinaryChunk => {
                write!(
                    f,
                    "GlTF using the 1.0 specification must have the start of the binary chunk aligned to 4 bytes."
                )
            }
            Self::Json(json) => {
                writeln!(f, "Could not decode Gltf's JSON.")?;
                write!(f, "{json}")
            }
        }
    }
}

impl core::error::Error for GltfValidationError {}

impl From<JsonParsingError> for GltfValidationError {
    fn from(value: JsonParsingError) -> Self {
        Self::Json(value)
    }
}
