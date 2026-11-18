mod error;
mod stack;
mod state;
mod tasks;

use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use log::*;

use crate::{
    Json, JsonObject, JsonValue, POINTER_WIDTH,
    object::JsonKeyValue,
    parser::{
        error::JsonParsingErrorType, stack::JsonParserStack, state::JsonParserState,
        tasks::JsonParserTask,
    },
    str_cursor::StrCursor,
    tokenizer::JsonTokenizer,
};

pub use error::JsonParsingError;

pub struct JsonParser;

impl JsonParser {
    /// Parses a JSON file.
    ///
    /// Can only take UTF-8 inputs without BOM, convert your data
    /// to UTF-8 before passing it to this method.
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
        let mut tokens = JsonTokenizer::new(data.chars());
        let mut state = JsonParserState::Root;
        let mut stack = alloc::vec![];
        let mut tasks = alloc::vec![];

        for token in &mut tokens {
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

        if !stack.is_empty() {
            let end = tokens.cursor();
            return match stack.last().unwrap() {
                JsonParserStack::Object { start, .. } => Err(JsonParsingError {
                    json: data,
                    start: *start,
                    end,
                    error_type: JsonParsingErrorType::UnclosedObject,
                }),
                JsonParserStack::Array { start, .. } => Err(JsonParsingError {
                    json: data,
                    start: *start,
                    end,
                    error_type: JsonParsingErrorType::UnclosedArray,
                }),
            };
        }

        Ok((tasks, data))
    }

    fn compile_json(json_data: &str, tasks: Vec<JsonParserTask>) -> Box<[u8]> {
        let CompiledJsonSizes {
            json_values_count,
            json_values_size,
            array_values_count,
            array_values_size,
            object_pairs_count,
            object_pairs_size,
        } = Self::calculate_size_of_compiled_json(&tasks);

        let mut data = Vec::from_iter(
            [0u8]
                .iter()
                .cycle()
                .copied()
                .take(json_values_size + array_values_size + object_pairs_size),
        );

        let mut deferred_objects = Vec::new();
        let mut deferred_arrays = Vec::new();

        let json_values_array = unsafe {
            core::slice::from_raw_parts_mut(
                data.as_mut_ptr().cast::<JsonValue>(),
                json_values_count,
            )
        };
        Self::compile_tasks(
            json_data,
            json_values_array,
            tasks,
            &mut deferred_objects,
            &mut deferred_arrays,
        );

        let json_object_values = unsafe {
            core::slice::from_raw_parts_mut(
                data.as_mut_ptr()
                    .byte_add(json_values_size)
                    .cast::<JsonKeyValue>(),
                object_pairs_count,
            )
        };
        Self::compile_objects(json_values_array, json_object_values, deferred_objects);

        let json_values_array = unsafe {
            core::slice::from_raw_parts_mut(
                data.as_mut_ptr().cast::<JsonValue>(),
                json_values_count,
            )
        };
        let json_array_values = unsafe {
            core::slice::from_raw_parts_mut(
                data.as_mut_ptr()
                    .byte_add(json_values_size + object_pairs_size)
                    .cast::<&JsonValue>(),
                array_values_count,
            )
        };
        Self::compile_arrays(json_values_array, json_array_values, deferred_arrays);

        data.into_boxed_slice()
    }

    fn compile_tasks<'a>(
        json_data: &'a str,
        json_values: &mut [JsonValue<'a>],
        tasks: Vec<JsonParserTask>,
        deferred_objects: &mut Vec<(usize, Vec<(usize, usize)>)>,
        deferred_arrays: &mut Vec<(usize, Vec<usize>)>,
    ) {
        for (i, (task, json_value)) in tasks.into_iter().zip(json_values).enumerate() {
            match task {
                JsonParserTask::Placeholder => {
                    unreachable!("Placeholder tasks should not reach compilation.");
                }
                JsonParserTask::Object { pairs } => {
                    deferred_objects.push((i, pairs));
                }
                JsonParserTask::Array { values } => {
                    deferred_arrays.push((i, values));
                }
                JsonParserTask::String { start, end } => {
                    let len = end.byte - start.byte - 2;
                    let offset = start.byte + 1;

                    *json_value = JsonValue::String(&json_data[offset..(offset + len)]);
                }
                JsonParserTask::Number { start, end } => {
                    let len = end.byte - start.byte;
                    let offset = start.byte;

                    *json_value = JsonValue::Number(&json_data[offset..(offset + len)]);
                }
                JsonParserTask::True => {
                    *json_value = JsonValue::True;
                }
                JsonParserTask::False => {
                    *json_value = JsonValue::False;
                }
                JsonParserTask::Null => {
                    *json_value = JsonValue::Null;
                }
            }
        }
    }

    fn compile_objects<'a>(
        json_values: &'a mut [JsonValue<'a>],
        mut object_values: &'a mut [JsonKeyValue<'a>],
        deferred_objects: Vec<(usize, Vec<(usize, usize)>)>,
    ) {
        let len = json_values.len();
        for (index, deferred_object) in deferred_objects {
            let (head, tail) = object_values.split_at_mut(deferred_object.len());
            object_values = tail;

            let mut pairs = deferred_object
                .iter()
                .map(|(key_index, value_index)| {
                    assert!(*key_index < len);
                    let key_ref =
                        unsafe { json_values.as_ptr().add(*key_index).as_ref_unchecked() };
                    let JsonValue::String(key) = key_ref else {
                        panic!("Object key was not String.");
                    };
                    assert!(*value_index < len);
                    let value =
                        unsafe { json_values.as_ptr().add(*value_index).as_ref_unchecked() };
                    JsonKeyValue { key, value }
                })
                .collect::<Vec<_>>();
            pairs.sort_by_cached_key(|JsonKeyValue { key, value: _ }| *key);

            for (dest, pair) in head.iter_mut().zip(pairs) {
                *dest = pair;
            }

            assert!(index < len);
            unsafe {
                json_values
                    .as_mut_ptr()
                    .add(index)
                    .write(JsonValue::Object(JsonObject { pairs: head }))
            };
        }
    }

    fn compile_arrays<'a>(
        json_values: &'a mut [JsonValue<'a>],
        mut array_values: &'a mut [&'a JsonValue<'a>],
        deferred_arrays: Vec<(usize, Vec<usize>)>,
    ) {
        let len = json_values.len();
        for (index, deferred_array) in deferred_arrays {
            let (head, tail) = array_values.split_at_mut(deferred_array.len());
            array_values = tail;

            for (dest, source_index) in head.iter_mut().zip(deferred_array) {
                assert!(source_index < len);
                *dest = unsafe { json_values.as_ptr().add(source_index).as_ref_unchecked() };
            }

            assert!(index < len);
            unsafe {
                json_values
                    .as_mut_ptr()
                    .add(index)
                    .write(JsonValue::Array { values: head })
            };
        }
    }

    fn calculate_size_of_compiled_json(tasks: &[JsonParserTask]) -> CompiledJsonSizes {
        let mut size = CompiledJsonSizes {
            json_values_count: 0,
            json_values_size: 0,
            array_values_count: 0,
            array_values_size: 0,
            object_pairs_count: 0,
            object_pairs_size: 0,
        };

        let len = tasks.len();
        size.json_values_count = len;
        size.json_values_size = len * size_of::<JsonValue>();

        for task in tasks {
            match task {
                JsonParserTask::Object { pairs } => {
                    let len = pairs.len();
                    size.object_pairs_count += len;
                    size.object_pairs_size += len * POINTER_WIDTH * 3;
                }
                JsonParserTask::Array { values } => {
                    let len = values.len();
                    size.array_values_count += len;
                    size.array_values_size += len * POINTER_WIDTH;
                }
                _ => (),
            };
        }

        size
    }
}

struct CompiledJsonSizes {
    json_values_count: usize,
    json_values_size: usize,
    array_values_count: usize,
    array_values_size: usize,
    object_pairs_count: usize,
    object_pairs_size: usize,
}
