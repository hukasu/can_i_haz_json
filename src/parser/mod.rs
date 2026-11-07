mod error;
mod stack;
mod state;
mod tasks;

use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use log::*;

use crate::{
    FIRST_OFFSET, Json, JsonValue, POINTER_WIDTH, SECOND_OFFSET, THIRD_OFFSET,
    parser::{error::JsonParsingErrorType, state::JsonParserState, tasks::JsonParserTask},
    str_cursor::StrCursor,
    tokenizer::JsonTokenizer,
};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(data: Cow<'a, str>) -> Result<Json<'a>, JsonParsingError<'a>> {
        trace!("Parsing json.");
        let (tasks, data) = Self::parse_json(data)?;

        if tasks.is_empty() {
            Err(JsonParsingError {
                json: data,
                start: StrCursor::default(),
                end: StrCursor::default(),
                error_type: JsonParsingErrorType::EmptyBody,
            })
        } else {
            let compiled = Self::compile_json(&data, tasks);
            Ok(Json { data, compiled })
        }
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

    fn compile_json(json_data: &str, tasks: Vec<JsonParserTask>) -> Box<[u8]> {
        let size = Self::calculate_size_of_compiled_json(&tasks);

        let mut data = Vec::from_iter([0u8].iter().cycle().copied().take(size));

        let json_base = json_data.as_ptr();
        let data_base = data.as_mut_ptr().cast::<JsonValue>();

        let mut deferred_arrays = Vec::new();

        let mut data_cursor = data.as_mut_slice();
        data_cursor = Self::compile_tasks(json_base, data_cursor, tasks, &mut deferred_arrays);
        data_cursor = Self::compile_arrays(data_base, data_cursor, deferred_arrays);
        assert!(
            data_cursor.is_empty(),
            "There was {} bytes left on compiled json buffer.",
            data_cursor.len()
        );

        data.into_boxed_slice()
    }

    fn compile_tasks<'a>(
        json_base: *const u8,
        mut data_cursor: &'a mut [u8],
        tasks: Vec<JsonParserTask>,
        deferred_arrays: &mut Vec<(usize, Vec<usize>)>,
    ) -> &'a mut [u8] {
        for (i, task) in tasks.into_iter().enumerate() {
            match task {
                JsonParserTask::Placeholder => {
                    unreachable!("Placeholder tasks should not reach compilation.");
                }
                JsonParserTask::Object => {
                    data_cursor[0] = 0;
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::Array { values } => {
                    deferred_arrays.push((i, values));
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::String { start, end } => {
                    let len = end.byte - start.byte - 2;
                    let offset = start.byte + 1;

                    data_cursor[0] = 2;
                    data_cursor[FIRST_OFFSET..SECOND_OFFSET].copy_from_slice(
                        &unsafe { json_base.byte_add(offset) }.addr().to_ne_bytes(),
                    );
                    data_cursor[SECOND_OFFSET..THIRD_OFFSET].copy_from_slice(&len.to_ne_bytes());
                    data_cursor = &mut data_cursor[0x18..];
                }
                JsonParserTask::Number { start, end } => {
                    let len = end.byte - start.byte;
                    let offset = start.byte;

                    data_cursor[0] = 3;
                    data_cursor[FIRST_OFFSET..SECOND_OFFSET].copy_from_slice(
                        &unsafe { json_base.byte_add(offset) }.addr().to_ne_bytes(),
                    );
                    data_cursor[SECOND_OFFSET..THIRD_OFFSET].copy_from_slice(&len.to_ne_bytes());
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
        data_cursor
    }

    fn compile_arrays<'a>(
        data_base: *mut JsonValue,
        mut data_cursor: &'a mut [u8],
        deferred_arrays: Vec<(usize, Vec<usize>)>,
    ) -> &'a mut [u8] {
        for (index, deferred_array) in deferred_arrays {
            let array_base = data_cursor.as_ptr().cast();

            unsafe {
                data_base.add(index).write(JsonValue::Array {
                    values: core::slice::from_raw_parts(array_base, deferred_array.len()),
                })
            };

            for value in deferred_array {
                unsafe {
                    data_cursor
                        .as_mut_ptr()
                        .cast::<*const JsonValue>()
                        .write(data_base.add(value))
                };
                data_cursor = &mut data_cursor[POINTER_WIDTH..];
            }
        }
        data_cursor
    }

    fn calculate_size_of_compiled_json(tasks: &[JsonParserTask]) -> usize {
        let mut size = tasks.len() * size_of::<JsonValue>();

        for task in tasks {
            let extra = match task {
                JsonParserTask::Object => 0,
                JsonParserTask::Array { values } => values.len() * POINTER_WIDTH,
                _ => 0,
            };

            size += extra
        }

        size
    }
}
