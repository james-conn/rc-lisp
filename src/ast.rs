#[derive(Debug)]
pub struct LispProgram {
	statements: Vec<LispAst>
}

impl From<Vec<LispAst>> for LispProgram {
	fn from(statements: Vec<LispAst>) -> Self {
		Self { statements }
	}
}

impl LispProgram {
	pub fn statements(&self) -> &[LispAst] {
		&self.statements
	}
}

#[derive(Debug)]
pub enum LispAst {
	Var(String),
	CallFunc { func: Box<LispAst>, args: Vec<LispAst> },
	LetBind { name: String, bind_expr: Box<LispAst>, in_expr: Box<LispAst> },
	Lambda { arg_name: String, body: Box<LispAst> },
	Int(i64)
}
