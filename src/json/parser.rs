use alloc::{boxed::Box, rc::Rc, string::String, sync::Arc};
use core::iter::Peekable;

use crate::json::{JsonDocument, data::JsonData};

impl JsonDocument<Box<JsonData>> {
    pub fn parse(contents: &str) -> Self {
        Self {
            data: Box::new(
                JsonParser {
                    iterator: contents.chars().peekable(),
                    column: 0,
                    line: 1,
                    buffer: String::with_capacity(16),
                }
                .parse(),
            ),
        }
    }
}

impl JsonDocument<Rc<JsonData>> {
    pub fn parse(contents: &str) -> Self {
        Self {
            data: Rc::new(
                JsonParser {
                    iterator: contents.chars().peekable(),
                    column: 0,
                    line: 1,
                    buffer: String::with_capacity(16),
                }
                .parse(),
            ),
        }
    }
}

impl JsonDocument<Arc<JsonData>> {
    pub fn parse(contents: &str) -> Self {
        Self {
            data: Arc::new(
                JsonParser {
                    iterator: contents.chars().peekable(),
                    column: 0,
                    line: 1,
                    buffer: String::with_capacity(16),
                }
                .parse(),
            ),
        }
    }
}

struct JsonParser<T: Iterator<Item = char>> {}

struct JsonParserCharProcessing {
    char_decision: CharDecision,
}

/// What should be done to the current character
enum CharDecision {
    /// Keep in the iterator
    Keep,
    /// Ignore, this will pop the char from the iterator but not include on buffer
    Ignore,
    /// Include on the buffer
    Include,
}

pub enum JsonParsingError {}

impl<T: Iterator<Item = char>> JsonParser<T> {
    pub fn parse(&mut self) -> JsonData {
        todo!()
    }
}

fn start() {}
