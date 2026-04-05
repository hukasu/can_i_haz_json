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
//! let data: &[u8] = b"{}";
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
//! Since it takes a type that only requires [`Borrow<[u8]>`](core::borrow::Borrow),
//! it will be tied to lifetime of the object that is given to [`Gltf::new`].
//!
//! ```rust,compile_fail
//! # use can_i_haz_gltf::Gltf;
//! // Fails because `data` does not live long enough.
//! let gltf = {
//!     let data: Vec<u8> = b"{}".to_vec();
//!     Gltf::new(data.as_slice()).unwrap()
//! };
//! let _  = gltf.version();
//! ```
//!

extern crate alloc;

pub mod json;

use core::{borrow::Borrow, fmt::Display};

use crate::json::{JsonDocument, JsonError};

const GLTF_MAGIC: [u8; 4] = [0x67, 0x6C, 0x54, 0x46];
const GLTF_JSON_CHUNK_TYPE: u32 = 0x4E4F534A;
const GLTF_BIN_CHUNK_TYPE: u32 = 0x004E4942;

#[derive(Clone, PartialEq, Eq)]
pub struct Gltf<T: Borrow<[u8]> + Clone> {
    version: u32,
    length: u32,
    json_start: usize,
    json_length: usize,
    binary_start: usize,
    binary_length: usize,
    json_document: JsonDocument<T>,
}

impl<T: Borrow<[u8]> + Clone> Gltf<T> {
    pub fn new(contents: T) -> Result<Gltf<T>, GltfValidationError> {
        let contents_borrow = contents.borrow();
        if contents_borrow[0] == b'{' {
            Self::gltf_new(contents)
        } else if contents_borrow[..4] != GLTF_MAGIC {
            Err(GltfValidationError::WrongMagic)
        } else {
            Self::glb_new(contents)
        }
    }

    /// Returns the version of the specification used to generate the
    /// [`Gltf`]. This is only populated on binary `.glb`, returning `None`
    /// for `.gltf` files. The specification of a `.gltf` file can be
    /// queried through the `asset` field of the document.
    pub fn version(&self) -> Option<u32> {
        Some(self.version).filter(|version| *version != 0)
    }

    fn gltf_new(contents: T) -> Result<Gltf<T>, GltfValidationError> {
        let contents_borrow = contents.borrow();
        let length = contents_borrow.len();
        let Ok(length_u32) = u32::try_from(length) else {
            unreachable!("GlTF json should never be larger then 4GB.");
        };

        let json_document = JsonDocument::new(contents)?;

        // TODO take the version from the json

        Ok(Self {
            version: 0,
            length: length_u32,
            json_start: 0,
            json_length: length,
            binary_start: length,
            binary_length: 0,
            json_document,
        })
    }

    fn glb_new(contents: T) -> Result<Gltf<T>, GltfValidationError> {
        let contents_borrow = contents.borrow();
        let version = u32::from_le_bytes(contents_borrow[4..8].try_into().unwrap());
        let length = u32::from_le_bytes(contents_borrow[8..12].try_into().unwrap());

        if version == 1 {
            Self::glb_1_new(contents, version, length)
        } else if version == 2 {
            Self::glb_2_new(contents, version, length)
        } else {
            Err(GltfValidationError::UnknownVersion)
        }
    }

    /// Implementation of [KHR_binary_glTF](https://github.com/KhronosGroup/glTF/blob/main/extensions/1.0/Khronos/KHR_binary_glTF/README.md)
    fn glb_1_new(contents: T, version: u32, length: u32) -> Result<Gltf<T>, GltfValidationError> {
        let contents_borrow = contents.borrow();
        let Ok(length_usize) = usize::try_from(length) else {
            return Err(GltfValidationError::Unaddressable);
        };

        let Ok(json_length) = usize::try_from(u32::from_le_bytes(
            contents_borrow[12..16].try_into().unwrap(),
        )) else {
            return Err(GltfValidationError::Unaddressable);
        };
        if json_length % 4 != 0 {
            return Err(GltfValidationError::UnalignedBinaryChunk);
        }
        let json_chunk_type = u32::from_le_bytes(contents_borrow[16..20].try_into().unwrap());
        if json_chunk_type != 0 {
            return Err(GltfValidationError::UnknownChunk {
                chunk: 0,
                chunk_type: json_chunk_type,
            });
        }

        let binary_start = 20 + json_length;
        let binary_length = length_usize - binary_start;

        Ok(Self {
            version,
            length,
            json_start: 20,
            json_length,
            binary_start,
            binary_length,
            json_document: JsonDocument::new(contents)?,
        })
    }

    fn glb_2_new(contents: T, version: u32, length: u32) -> Result<Gltf<T>, GltfValidationError> {
        let contents_borrow = contents.borrow();
        let Ok(length_usize) = usize::try_from(length) else {
            return Err(GltfValidationError::Unaddressable);
        };

        let Ok(json_length) = usize::try_from(u32::from_le_bytes(
            contents_borrow[12..16].try_into().unwrap(),
        )) else {
            return Err(GltfValidationError::Unaddressable);
        };
        let json_chunk_type = u32::from_le_bytes(contents_borrow[16..20].try_into().unwrap());
        if !Self::valid_chunk_type(json_chunk_type) {
            return Err(GltfValidationError::UnknownChunk {
                chunk: 0,
                chunk_type: json_chunk_type,
            });
        } else if json_chunk_type != GLTF_JSON_CHUNK_TYPE {
            return Err(GltfValidationError::MissingJsonChunk);
        }

        let binary_chunk_header = 20 + json_length;
        let (binary_start, binary_length) = if binary_chunk_header == length_usize {
            (length_usize, 0)
        } else {
            let Ok(binary_length) = usize::try_from(u32::from_le_bytes(
                contents_borrow[binary_chunk_header..(binary_chunk_header + 4)]
                    .try_into()
                    .unwrap(),
            )) else {
                return Err(GltfValidationError::Unaddressable);
            };
            let binary_chunk_type = u32::from_le_bytes(
                contents_borrow[(binary_chunk_header + 4)..(binary_chunk_header + 8)]
                    .try_into()
                    .unwrap(),
            );
            if !Self::valid_chunk_type(binary_chunk_type) {
                return Err(GltfValidationError::UnknownChunk {
                    chunk: 1,
                    chunk_type: binary_chunk_type,
                });
            }
            (binary_chunk_header + 8, binary_length)
        };

        Ok(Self {
            version,
            length,
            json_start: 20,
            json_length,
            binary_start,
            binary_length,
            json_document: JsonDocument::new(contents)?,
        })
    }

    const fn valid_chunk_type(chunk_type: u32) -> bool {
        chunk_type == GLTF_JSON_CHUNK_TYPE || chunk_type == GLTF_BIN_CHUNK_TYPE
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
    Json(JsonError),
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

impl From<JsonError> for GltfValidationError {
    fn from(value: JsonError) -> Self {
        Self::Json(value)
    }
}
