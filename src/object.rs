use crate::JsonValue;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct JsonObject<'a> {
    pub(crate) pairs: &'a [JsonKeyValue<'a>],
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct JsonKeyValue<'a> {
    pub(crate) key: &'a str,
    pub(crate) value: &'a JsonValue<'a>,
}

impl<'a> JsonObject<'a> {
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.len() == 0
    }

    #[must_use]
    pub fn find(&self, key: &str) -> Option<&'a JsonValue<'a>> {
        let found = self
            .pairs
            .binary_search_by_key(&key, |&JsonKeyValue { key, value: _ }| key)
            .ok()?;

        Some(self.pairs[found].value)
    }
}
