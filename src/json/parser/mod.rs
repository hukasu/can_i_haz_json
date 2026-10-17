mod error;

use log::*;

use crate::json::{
    JsonValue,
    tokenizer::{JsonTokenizer, TokenType},
};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a, T: Iterator<Item = char> + 'a>(
        tokens: JsonTokenizer<T>,
    ) -> Result<JsonValue<'a>, JsonParsingError> {
        trace!("Parsing json.");
        // let
        let mut stack = alloc::vec![];

        for token in tokens {
            let token = token?;
            trace!("Parsing json token {:?}.", token);
            match token.token_type {
                TokenType::LBracket => {}
                TokenType::String => match stack.last_mut() {
                    Some(JsonValue::Array) => {
                        unimplemented!()
                    }
                    None => {}
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }

        unimplemented!()
    }
}
