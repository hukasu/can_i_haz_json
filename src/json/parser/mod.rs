mod error;
mod state;
mod tasks;

use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use log::*;

use crate::json::{
    Json,
    parser::{error::JsonParsingErrorType, state::JsonParserState, tasks::JsonParserTask},
    tokenizer::JsonTokenizer,
};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(data: Cow<'a, str>) -> Result<Json<'a>, JsonParsingError<'a>> {
        trace!("Parsing json.");
        let (_tasks, data) = Self::parse_json(data)?;

        Ok(Json {
            data,
            compiled: Box::default(),
        })
    }

    fn parse_json<'a>(
        data: Cow<'a, str>,
    ) -> Result<(Vec<JsonParserTask>, Cow<'a, str>), JsonParsingError<'a>> {
        let tokens = JsonTokenizer::new(data.chars());
        let mut state = JsonParserState::Root;
        let mut stack = alloc::vec![];
        let mut tasks = alloc::vec![];

        for token in tokens {
            let token = match token {
                Ok(token) => token,
                Err(error) => {
                    return Err(JsonParsingError {
                        json: data,
                        start: error.start,
                        end: error.end,
                        error_type: JsonParsingErrorType::Tokenizer(error.error_type),
                    });
                }
            };

            trace!("Parsing json token {:?}.", token);
            match state.process(&mut tasks, &mut stack, &token) {
                Ok(new_state) => state = new_state,
                Err(error_type) => {
                    return Err(JsonParsingError {
                        json: data,
                        start: token.start,
                        end: token.end,
                        error_type,
                    });
                }
            }
        }

        Ok((tasks, data))
    }
}
