use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
	Parse(ParseArgs),
	Exec(ExecArgs)
}

#[derive(Args)]
pub struct ParseArgs {
	pub in_file: std::path::PathBuf
}

#[derive(Args)]
pub struct ExecArgs {
	pub in_file: std::path::PathBuf
}
