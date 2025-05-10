use crate::ast::LispAst;
use crate::val::LispVal;

// this can be changed if desired
pub type BuiltinFunc = fn(&LispAst) -> LispVal;
