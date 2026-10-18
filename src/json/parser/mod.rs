mod error;
mod state;
mod tasks;

use alloc::borrow::Cow;
use log::*;

use crate::json::{
    Json,
    parser::{error::JsonParsingErrorType, state::JsonParserState},
    tokenizer::{JsonTokenizer, Token},
};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(json: Cow<'a, str>) -> Result<Json<'a>, JsonParsingError<'a>> {
        let tokens = JsonTokenizer::new(json.chars());

        trace!("Parsing json.");
        let mut state = JsonParserState::Root;
        let mut stack = alloc::vec![];

        for token in tokens {
            let token = match token {
                Ok(token) => token,
                Err(error) => {
                    return Err(JsonParsingError {
                        json,
                        start: error.start,
                        end: error.end,
                        error_type: JsonParsingErrorType::Tokenizer(error.error_type),
                    });
                }
            };

            trace!("Parsing json token {:?}.", token);
            match state.process(&mut stack, &token) {
                Ok(new_state) => state = new_state,
                Err(error_type) => {
                    return Err(JsonParsingError {
                        json,
                        start: token.start,
                        end: token.end,
                        error_type,
                    });
                }
            }
        }

        unimplemented!()
    }
}
