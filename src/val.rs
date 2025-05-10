use crate::ast::LispAst;
use crate::builtins::BuiltinFunc;

// very much subject to change
#[derive(Debug)]
pub enum LispVal {
	Int(i64),
	Func { body: Box<LispAst>, arg_name: String },
	BuiltinFunc(BuiltinFunc)
}
