pub mod error;

pub use super::lexer;
pub use self::error::*;

pub type ParserResult<T> = Result<T, ParserError>;
