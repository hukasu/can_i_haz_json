use core::fmt::Display;

use crate::{GLTF_BIN_CHUNK_TYPE, GLTF_JSON_CHUNK_TYPE, json::JsonParsingError};

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
