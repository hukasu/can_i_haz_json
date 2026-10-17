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
        stack: &mut Vec<JsonParserTask>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        match self {
            Self::Root => Self::root(stack, token),
            Self::Object => unimplemented!(),
            Self::WaitingEof => unimplemented!(),
        }
    }

    fn root(stack: &mut Vec<JsonParserTask>, token: &Token) -> Result<Self, JsonParsingErrorType> {
        let Token {
            start,
            end,
            token_type,
        } = token;

        match token_type {
            TokenType::LBracket => {
                stack.push(JsonParserTask::Object);
                Ok(Self::Object)
            }
            TokenType::String => {
                stack.push(JsonParserTask::String {
                    start: *start,
                    end: *end,
                });
                Ok(Self::WaitingEof)
            }
            TokenType::RBracket => Err(JsonParsingErrorType::UnexpectedRCurly),
            _ => unimplemented!(),
        }
    }
}
