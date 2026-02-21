mod array;
mod data;
mod number;
mod object;
mod parser;
mod string;
mod tokenizer;
mod value;

use alloc::{borrow::Cow, boxed::Box, rc::Rc, sync::Arc};
use core::{error::Error, fmt::Display, ops::Deref};

use self::data::JsonData;
pub use self::{
    array::JsonArray,
    number::JsonNumber,
    object::JsonObject,
    parser::JsonParsingError,
    string::JsonString,
    value::{JsonValue, JsonValueType},
};

pub struct JsonDocument<C: Deref<Target = JsonData>> {
    data: C,
}

pub type BoxJsonDocument = JsonDocument<Box<JsonData>>;
pub type RcJsonDocument = JsonDocument<Rc<JsonData>>;
pub type ArcJsonDocument = JsonDocument<Arc<JsonData>>;

impl<C: Clone + Deref<Target = JsonData>> Clone for JsonDocument<C> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<C: Default + Deref<Target = JsonData>> Default for JsonDocument<C> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct WrongType<'a> {
    key: Cow<'a, str>,
    wrong: JsonValueType,
    correct: JsonValueType,
}

impl Display for WrongType<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Tried to retrieve {} as {:?}, but it is {:?}.",
            self.key, self.wrong, self.correct
        )
    }
}

impl Error for WrongType<'_> {}
