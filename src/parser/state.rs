use alloc::vec::Vec;
use log::trace;

use crate::{
    parser::{error::JsonParsingErrorType, stack::JsonParserStack, tasks::JsonParserTask},
    tokenizer::{Token, TokenType},
};

pub enum JsonParserState {
    Root,
    Object(TokenType),
    ObjectWithKey(usize),
    ObjectWaitingValue(usize),
    ObjectSeparator,
    Array,
    ArraySeparator,
    WaitingEof,
}

impl JsonParserState {
    pub fn process(
        &self,
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        match self {
            Self::Root => Self::root(tasks, stack, token),
            Self::Object(previous_token) => Self::object(*previous_token, tasks, stack, token),
            Self::ObjectWithKey(key_index) => {
                Self::object_with_key(*key_index, tasks, stack, token)
            }
            Self::ObjectWaitingValue(key_index) => {
                Self::object_waiting_value(*key_index, tasks, stack, token)
            }
            Self::ObjectSeparator => Self::object_separator(tasks, stack, token),
            Self::Array => Self::array(tasks, stack, token),
            Self::ArraySeparator => Self::array_separator(tasks, stack, token),
            Self::WaitingEof => Err(JsonParsingErrorType::MultipleRoots),
        }
    }

    fn root(
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'root' consumed {token:?}");
        let Token {
            start,
            end,
            token_type,
        } = *token;

        match token_type {
            TokenType::LCurly => {
                let position = tasks.len();
                tasks.push(JsonParserTask::Placeholder);
                stack.push(JsonParserStack::Object {
                    start,
                    placeholder: position,
                    pairs: Vec::new(),
                });
                Ok(Self::Object(token_type))
            }
            TokenType::RCurly => Err(JsonParsingErrorType::UnexpectedRCurly),
            TokenType::LBracket => {
                let position = tasks.len();
                tasks.push(JsonParserTask::Placeholder);
                stack.push(JsonParserStack::Array {
                    start,
                    placeholder: position,
                    values: Vec::new(),
                });
                Ok(Self::Array)
            }
            TokenType::RBracket => Err(JsonParsingErrorType::UnexpectedRBracket),
            TokenType::Colon => unimplemented!(),
            TokenType::Comma => unimplemented!(),
            TokenType::String => {
                tasks.push(JsonParserTask::String { start, end });
                Ok(Self::WaitingEof)
            }
            TokenType::Number => {
                tasks.push(JsonParserTask::Number { start, end });
                Ok(Self::WaitingEof)
            }
            TokenType::True => {
                tasks.push(JsonParserTask::True);
                Ok(Self::WaitingEof)
            }
            TokenType::False => {
                tasks.push(JsonParserTask::False);
                Ok(Self::WaitingEof)
            }
            TokenType::Null => {
                tasks.push(JsonParserTask::Null);
                Ok(Self::WaitingEof)
            }
            TokenType::Eof => unimplemented!(),
        }
    }

    fn object(
        previous_token: TokenType,
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'object' consumed {token:?}");
        let Token {
            start,
            end,
            token_type,
        } = *token;

        match token_type {
            TokenType::LCurly => Err(JsonParsingErrorType::InvalidObjectKey(token_type)),
            TokenType::RCurly => match previous_token {
                TokenType::LCurly => {
                    let last = stack.pop();

                    if let Some(JsonParserStack::Object {
                        start: _,
                        placeholder,
                        pairs,
                    }) = last
                    {
                        tasks[placeholder] = JsonParserTask::Object { pairs };
                    } else {
                        return Err(JsonParsingErrorType::UnmatchedRCurly);
                    }

                    Self::stack_based_next_state(stack)
                }
                TokenType::Comma => Err(JsonParsingErrorType::ObjectTrailingComma),
                _ => {
                    unreachable!(
                        "Object state can never be reached from a {:?} token.",
                        previous_token
                    )
                }
            },
            TokenType::LBracket => Err(JsonParsingErrorType::InvalidObjectKey(token_type)),
            TokenType::RBracket => Err(JsonParsingErrorType::UnexpectedRBracket),
            TokenType::Comma => Err(JsonParsingErrorType::CommaBeforeObjectKeyValue),
            TokenType::Colon => Err(JsonParsingErrorType::CommaBeforeObjectKey),
            TokenType::String => {
                let index = tasks.len();
                tasks.push(JsonParserTask::String { start, end });

                Ok(Self::ObjectWithKey(index))
            }
            TokenType::Number | TokenType::True | TokenType::False | TokenType::Null => {
                Err(JsonParsingErrorType::InvalidObjectKey(token_type))
            }
            TokenType::Eof => Self::eof(),
        }
    }

    fn object_with_key(
        key_index: usize,
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'object_with_key' consumed {token:?}");
        let Token {
            start: _,
            end: _,
            token_type,
        } = *token;

        match token_type {
            TokenType::Colon => Ok(Self::ObjectWaitingValue(key_index)),
            _ => unimplemented!(),
        }
    }

    fn object_waiting_value(
        key_index: usize,
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'object_waiting_value' consumed {token:?}");
        let Token {
            start,
            end,
            token_type,
        } = *token;

        match token_type {
            TokenType::LCurly => {
                let position = tasks.len();
                tasks.push(JsonParserTask::Placeholder);
                stack.push(JsonParserStack::Object {
                    start,
                    placeholder: position,
                    pairs: Vec::new(),
                });
                Ok(Self::Object(token_type))
            }
            TokenType::RCurly => Err(JsonParsingErrorType::UnexpectedRCurly),
            TokenType::LBracket => {
                let position = tasks.len();
                tasks.push(JsonParserTask::Placeholder);
                stack.push(JsonParserStack::Array {
                    start,
                    placeholder: position,
                    values: Vec::new(),
                });
                Ok(Self::Array)
            }
            TokenType::RBracket => Err(JsonParsingErrorType::UnexpectedRBracket),
            TokenType::Colon => Err(JsonParsingErrorType::DuplicateColon),
            TokenType::Comma => Err(JsonParsingErrorType::ObjectMissingValue),
            TokenType::String => {
                let Some(JsonParserStack::Object {
                    start: _,
                    placeholder: _,
                    pairs,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let val_index = tasks.len();

                tasks.push(JsonParserTask::String { start, end });
                pairs.push((key_index, val_index));

                Ok(Self::ObjectSeparator)
            }
            TokenType::Number => {
                let Some(JsonParserStack::Object {
                    start: _,
                    placeholder: _,
                    pairs,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let val_index = tasks.len();

                tasks.push(JsonParserTask::Number { start, end });
                pairs.push((key_index, val_index));

                Ok(Self::ObjectSeparator)
            }
            TokenType::True => {
                let Some(JsonParserStack::Object {
                    start: _,
                    placeholder: _,
                    pairs,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let val_index = tasks.len();

                tasks.push(JsonParserTask::True);
                pairs.push((key_index, val_index));

                Ok(Self::ObjectSeparator)
            }
            TokenType::False => {
                let Some(JsonParserStack::Object {
                    start: _,
                    placeholder: _,
                    pairs,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let val_index = tasks.len();

                tasks.push(JsonParserTask::False);
                pairs.push((key_index, val_index));

                Ok(Self::ObjectSeparator)
            }
            TokenType::Null => {
                let Some(JsonParserStack::Object {
                    start: _,
                    placeholder: _,
                    pairs,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let val_index = tasks.len();

                tasks.push(JsonParserTask::Null);
                pairs.push((key_index, val_index));

                Ok(Self::ObjectSeparator)
            }
            TokenType::Eof => Self::eof(),
        }
    }

    fn object_separator(
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'object_separator' consumed {token:?}");
        let Token {
            start: _,
            end: _,
            token_type,
        } = *token;

        match token_type {
            TokenType::RCurly => {
                let last = stack.pop();

                if let Some(JsonParserStack::Object {
                    start: _,
                    placeholder,
                    pairs,
                }) = last
                {
                    tasks[placeholder] = JsonParserTask::Object { pairs };
                } else {
                    return Err(JsonParsingErrorType::UnmatchedRCurly);
                }

                Self::stack_based_next_state(stack)
            }
            TokenType::RBracket => Err(JsonParsingErrorType::UnexpectedRBracket),
            TokenType::Comma => Ok(Self::Object(token_type)),
            TokenType::LCurly
            | TokenType::LBracket
            | TokenType::Colon
            | TokenType::String
            | TokenType::Number
            | TokenType::True
            | TokenType::False
            | TokenType::Null => Err(JsonParsingErrorType::ObjectExpectedComma),
            TokenType::Eof => Self::eof(),
        }
    }

    fn array(
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'array' consumed {token:?}");
        let Token {
            start,
            end,
            token_type,
        } = *token;

        match token_type {
            TokenType::LCurly => {
                let position = tasks.len();
                tasks.push(JsonParserTask::Placeholder);
                stack.push(JsonParserStack::Object {
                    start,
                    placeholder: position,
                    pairs: Vec::new(),
                });
                Ok(Self::Object(token_type))
            }
            TokenType::RCurly => Err(JsonParsingErrorType::UnexpectedRCurly),
            TokenType::LBracket => {
                let Some(JsonParserStack::Array {
                    start: _,
                    placeholder: _,
                    values,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let position = tasks.len();
                values.push(position);
                tasks.push(JsonParserTask::Placeholder);
                stack.push(JsonParserStack::Array {
                    start,
                    placeholder: position,
                    values: Vec::new(),
                });
                Ok(Self::Array)
            }
            TokenType::RBracket => {
                let last = stack.pop();

                if let Some(JsonParserStack::Array {
                    start: _,
                    placeholder,
                    values,
                }) = last
                {
                    if values.is_empty() {
                        tasks[placeholder] = JsonParserTask::Array { values };
                    } else {
                        return Err(JsonParsingErrorType::NoValueAfterArraySeparator);
                    }
                } else {
                    return Err(JsonParsingErrorType::UnmatchedRBracket);
                }

                Self::stack_based_next_state(stack)
            }
            TokenType::String => {
                let Some(JsonParserStack::Array {
                    start: _,
                    placeholder: _,
                    values,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let position = tasks.len();
                values.push(position);
                tasks.push(JsonParserTask::String { start, end });
                Ok(Self::ArraySeparator)
            }
            TokenType::Number => {
                let Some(JsonParserStack::Array {
                    start: _,
                    placeholder: _,
                    values,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let position = tasks.len();
                values.push(position);
                tasks.push(JsonParserTask::Number { start, end });
                Ok(Self::ArraySeparator)
            }
            TokenType::True => {
                let Some(JsonParserStack::Array {
                    start: _,
                    placeholder: _,
                    values,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let position = tasks.len();
                values.push(position);
                tasks.push(JsonParserTask::True);
                Ok(Self::ArraySeparator)
            }
            TokenType::False => {
                let Some(JsonParserStack::Array {
                    start: _,
                    placeholder: _,
                    values,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let position = tasks.len();
                values.push(position);
                tasks.push(JsonParserTask::False);
                Ok(Self::ArraySeparator)
            }
            TokenType::Null => {
                let Some(JsonParserStack::Array {
                    start: _,
                    placeholder: _,
                    values,
                }) = stack.last_mut()
                else {
                    todo!();
                };

                let position = tasks.len();
                values.push(position);
                tasks.push(JsonParserTask::Null);
                Ok(Self::ArraySeparator)
            }
            TokenType::Comma => Err(JsonParsingErrorType::UnexpectedComma),
            _ => unimplemented!("{:?}", token),
        }
    }

    fn array_separator(
        tasks: &mut Vec<JsonParserTask>,
        stack: &mut Vec<JsonParserStack>,
        token: &Token,
    ) -> Result<Self, JsonParsingErrorType> {
        trace!("'array_separator' consumed {token:?}");
        let Token {
            start: _,
            end: _,
            token_type,
        } = token;

        match token_type {
            TokenType::RBracket => {
                let last = stack.pop();

                if let Some(JsonParserStack::Array {
                    start: _,
                    placeholder,
                    values,
                }) = last
                {
                    tasks[placeholder] = JsonParserTask::Array { values };
                } else {
                    return Err(JsonParsingErrorType::UnmatchedRBracket);
                }

                Self::stack_based_next_state(stack)
            }
            TokenType::Comma => Ok(JsonParserState::Array),
            TokenType::LBracket
            | TokenType::LCurly
            | TokenType::String
            | TokenType::Number
            | TokenType::True
            | TokenType::False
            | TokenType::Null => Err(JsonParsingErrorType::EmptyBody),
            _ => unimplemented!("{:?}", token),
        }
    }

    fn stack_based_next_state(stack: &[JsonParserStack]) -> Result<Self, JsonParsingErrorType> {
        match stack.last() {
            Some(JsonParserStack::Object { .. }) => Ok(Self::ObjectSeparator),
            Some(JsonParserStack::Array { .. }) => Ok(Self::ArraySeparator),
            None => Ok(Self::WaitingEof),
        }
    }

    #[inline(always)]
    fn eof() -> ! {
        unreachable!("Eof should never be consumed.")
    }
}
