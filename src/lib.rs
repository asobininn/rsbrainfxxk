pub mod parser;
pub mod ast;
pub mod interpreter;

pub use crate::ast::*;

pub type Result<T> = anyhow::Result<T>;
