pub mod error;
pub mod ast;

pub use super::lexer;
pub use self::error::*;
pub use self::ast::*;

pub type ParserResult<T> = Result<T, ParserError>;
