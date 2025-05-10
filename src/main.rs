use clap::Parser;

mod cli;
use cli::{Cli, Command};

mod ast;
mod builtins;
mod val;

mod parse;
use parse::parse_file;

mod interp;
use interp::interp_program;

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Command::Parse(args) => {
			let parsed = parse_file(args.in_file).unwrap();
			println!("{parsed:#?}");
		}
		Command::Exec(args) => {
			let parsed = parse_file(args.in_file).unwrap();
			interp_program(parsed).unwrap();
		}
	}
}
