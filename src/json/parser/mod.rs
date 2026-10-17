mod error;
mod state;
mod tasks;

use log::*;

use crate::json::{JsonValue, parser::state::JsonParserState, tokenizer::JsonTokenizer};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a, T: Iterator<Item = char> + 'a>(
        tokens: JsonTokenizer<T>,
    ) -> Result<JsonValue<'a>, JsonParsingError<'a>> {
        trace!("Parsing json.");
        let mut state = JsonParserState::Root;
        let mut stack = alloc::vec![];

        for token in tokens {
            let token = token?;
            trace!("Parsing json token {:?}.", token);
            state = state.process(&mut stack, &token)?;
        }

        unimplemented!()
    }
}
