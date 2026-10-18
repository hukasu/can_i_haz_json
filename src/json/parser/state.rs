use alloc::vec::Vec;

use crate::json::{
    parser::{error::JsonParsingErrorType, tasks::JsonParserTask},
    tokenizer::{Token, TokenType},
};

pub enum JsonParserState {
    Root,
    Object,
    WaitingEof,
}

impl JsonParserState {
    pub fn process(
        &self,
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserTask>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        match self {
            Self::Root => Self::root(tasks, stack, token),
            Self::Object => Self::object(tasks, stack, token),
            Self::WaitingEof => unimplemented!(),
        }
    }

    fn root(
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserTask>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        let Token {
            start,
            end,
            token_type,
        } = token;

        match token_type {
            TokenType::LCurly => {
                stack.push(JsonParserTask::Object);
                Ok(Self::Object)
            }
            TokenType::String => {
                tasks.push(JsonParserTask::String {
                    start: *start,
                    end: *end,
                });
                Ok(Self::WaitingEof)
            }
            TokenType::RCurly => Err(JsonParsingErrorType::UnexpectedRCurly),
            _ => unimplemented!(),
        }
    }

    fn object(
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserTask>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        let Token {
            start: _,
            end: _,
            token_type,
        } = token;

        match token_type {
            TokenType::RCurly => {
                let last = stack.pop();

                if let Some(obj @ JsonParserTask::Object) = last {
                    tasks.push(obj);
                } else {
                    return Err(JsonParsingErrorType::UnmatchedRCurly);
                }

                Self::stack_based_next_state(stack)
            }
            _ => unimplemented!(),
        }
    }

    fn stack_based_next_state(stack: &[JsonParserTask]) -> Result<Self, JsonParsingErrorType> {
        match stack.last() {
            Some(JsonParserTask::Object) => Ok(Self::Object),
            Some(JsonParserTask::Array) => unimplemented!(),
            Some(JsonParserTask::String { .. }) => {
                unreachable!("String should never be present on stack.")
            }
            Some(JsonParserTask::Number { .. }) => {
                unreachable!("Number should never be present on stack.")
            }
            Some(JsonParserTask::True) => {
                unreachable!("True should never be present on stack.")
            }
            Some(JsonParserTask::False) => {
                unreachable!("False should never be present on stack.")
            }
            Some(JsonParserTask::Null) => {
                unreachable!("Null should never be present on stack.")
            }
            None => Ok(Self::WaitingEof),
        }
    }
}
