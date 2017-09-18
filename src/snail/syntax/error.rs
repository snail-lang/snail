use std::fmt;
use super::lexer::TokenPosition;

#[derive(Debug)]
pub enum RunErrorValue {
    Constant(String),
}

#[derive(Debug)]
pub struct RunError {
    value:    RunErrorValue,
    position: Option<TokenPosition>,
}

impl RunError {
    pub fn new(value: &str) -> RunError {
        RunError {
            value:    RunErrorValue::Constant(value.to_owned()),
            position: None,
        }
    }

    pub fn new_pos(position: TokenPosition, value: &str) -> RunError {
        RunError {
            value: RunErrorValue::Constant(value.to_owned()),
            position: Some(position),
        }
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            RunErrorValue::Constant(ref s) => match self.position {
                Some(p) => write!(f, "{}: {}", p, s),
                None    => write!(f, "{}", s),
            },
        }
    }
}
