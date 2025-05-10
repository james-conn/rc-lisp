use chumsky::prelude::*;
use ariadne::{Report, ReportKind, Label, Source, Color};
use crate::ast::{LispProgram, LispAst};

// some arithmatic operators plus `text::ascii::ident()`
fn lisp_ident<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> + Clone {
	just('+').to_slice()
		.or(just('-').to_slice())
		.or(just('*').to_slice())
		.or(just('/').to_slice())
		.or(text::ascii::ident())
		.padded()
}

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<LispAst>, extra::Err<Rich<'a, char>>> {
	let expr = recursive(|expr| {
		let num = just('-').or_not()
			.then(text::int(10))
			.to_slice()
			.map(|s: &str| s.parse().unwrap())
			.padded();

		let let_bind = just("let")
			.ignore_then(lisp_ident().delimited_by(just('('), just(')')).padded())
			.then(expr.clone())
			.then(expr.clone())
			.delimited_by(just('('), just(')'))
			.map(|((name, bind_expr), in_expr): ((&str, LispAst), LispAst)| {
				(name.to_string(), Box::new(bind_expr), Box::new(in_expr))
			})
			.padded();

		let lambda_def = just("lambda")
			.ignore_then(lisp_ident().delimited_by(just('('), just(')')).padded())
			.then(expr.clone())
			.delimited_by(just('('), just(')'))
			.map(|(arg_name, body): (&str, LispAst)| (arg_name.to_string(), Box::new(body)))
			.padded();

		let call_func = expr.clone()
			.then(
				expr.clone().repeated()
					.collect::<Vec<LispAst>>()
					.or_not()
			)
			.delimited_by(just('('), just(')'))
			.map(|(func, args): (LispAst, Option<Vec<LispAst>>)| (Box::new(func), args.unwrap_or(vec![])))
			.padded();

		choice((
			num.map(LispAst::Int),
			lisp_ident().map(str::to_string).map(LispAst::Var),
			let_bind.map(|(name, bind_expr, in_expr)| LispAst::LetBind { name, bind_expr, in_expr }),
			lambda_def.map(|(arg_name, body)| LispAst::Lambda { arg_name, body }),
			call_func.map(|(func, args)| LispAst::CallFunc { func, args })
		))
	});

	expr.repeated().collect::<Vec<_>>()
}

pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<LispProgram> {
	let path_str = path.as_ref().to_str()
		.ok_or(anyhow::anyhow!("non-unicode path provided"))?.to_owned();
	let code = std::fs::read_to_string(path)?;

	let result = parser().parse(&code).into_result().map_err(|errs| {
		for err in errs {
			Report::build(ReportKind::Error, (&path_str, err.span().into_range()))
				.with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
				.with_message(err.to_string())
				.with_label(
					Label::new((&path_str, err.span().into_range()))
						.with_message(err.reason().to_string())
						.with_color(Color::Red)
					)
				.finish().eprint((&path_str, Source::from(&code))).unwrap();
		}

		anyhow::anyhow!("parsing failed")
	}).map(LispProgram::from);

	result
}
