pub mod lexer;
pub mod parser;
pub mod error;
pub mod symtab;
pub mod typetab;

pub type RunResult<T> = Result<T, RunError>;

pub use self::parser::*;
pub use self::lexer::*;
pub use self::symtab::*;
pub use self::typetab::*;
pub use self::error::*;
