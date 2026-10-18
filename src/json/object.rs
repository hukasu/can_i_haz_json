use crate::json::JsonValue;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct JsonObject<'a> {
    pairs: &'a [JsonKeyValue<'a>],
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct JsonKeyValue<'a> {
    key: &'a str,
    value: &'a JsonValue<'a>,
}

impl<'a> JsonObject<'a> {
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.len() == 0
    }

    pub fn find(&self, key: &str) -> Option<&'a JsonValue<'a>> {
        let found = self
            .pairs
            .binary_search_by_key(&key, |&JsonKeyValue { key, value: _ }| key)
            .ok()?;

        Some(self.pairs[found].value)
    }
}
