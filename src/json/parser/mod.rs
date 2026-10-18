mod error;
mod state;
mod tasks;

use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use log::*;

use crate::json::{
    Json, JsonValue,
    parser::{error::JsonParsingErrorType, state::JsonParserState, tasks::JsonParserTask},
    tokenizer::JsonTokenizer,
};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(data: Cow<'a, str>) -> Result<Json<'a>, JsonParsingError<'a>> {
        trace!("Parsing json.");
        let (tasks, data) = Self::parse_json(data)?;
        let compiled = Self::compile_json(tasks);

        Ok(Json { data, compiled })
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

    fn compile_json(tasks: Vec<JsonParserTask>) -> Box<[u8]> {
        let size = Self::calculate_size_of_compiled_json(&tasks);

        let mut data = Vec::from_iter([0u8].iter().cycle().copied().take(size));

        let mut data_cursor = data.as_mut_slice();
        for task in tasks {
            match task {
                JsonParserTask::Object => {
                    data_cursor[0] = 0;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::Array => {
                    data_cursor[0] = 1;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::String { .. } => {
                    data_cursor[0] = 2;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::Number { .. } => {
                    data_cursor[0] = 3;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::True => {
                    data_cursor[0] = 4;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::False => {
                    data_cursor[0] = 5;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::Null => {
                    data_cursor[0] = 6;
                    data_cursor = &mut data_cursor[0x18..];
                }
            }
        }

        data.into_boxed_slice()
    }

    fn calculate_size_of_compiled_json(tasks: &[JsonParserTask]) -> usize {
        let mut size = tasks.len() * size_of::<JsonValue>();

        for task in tasks {
            let extra = match task {
                JsonParserTask::Object => 0,
                JsonParserTask::Array => 0,
                _ => 0,
            };

            size += extra
        }

        size
    }
}
