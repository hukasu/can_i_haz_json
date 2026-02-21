use core::fmt::Debug;

use alloc::vec::Vec;

use crate::json::{JsonArray, JsonNumber, JsonString, object::InternalJsonObject};

#[derive(Default)]
pub struct JsonData {
    strings: Vec<JsonString>,
    numbers: Vec<JsonNumber>,
    objects: Vec<InternalJsonObject>,
    arrays: Vec<JsonArray>,
}

impl Debug for JsonData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ strings: {}, numbers: {}, objects: {}, arrays: {} }}",
            self.strings.len(),
            self.numbers.len(),
            self.objects.len(),
            self.arrays.len()
        )
    }
}
