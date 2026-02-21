use alloc::{collections::BTreeMap, string::String};
use core::ops::Deref;

use crate::json::{JsonValue, data::JsonData};

pub struct JsonObject<'a, C: Deref<Target = JsonData>> {
    data: C,
    object: &'a InternalJsonObject,
}

impl<C: Deref<Target = JsonData>> Deref for JsonObject<'_, C> {
    type Target = InternalJsonObject;

    fn deref(&self) -> &Self::Target {
        self.object
    }
}

pub struct InternalJsonObject {
    values: BTreeMap<String, JsonValue>,
}
