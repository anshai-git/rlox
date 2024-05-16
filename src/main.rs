use clap::{command, Arg, ArgMatches, Command};

mod rlox;
mod scanner;
mod token;
mod token_type;
mod literal; // deprecated, instead: Object
mod object;
mod parser;
mod expr;
mod ast_printer;
mod interpreter;
mod stmt;
mod environment;
mod lox_callable;

use rlox::RLox;

fn main() {
    let command: Command = parse_args();
    let matches: ArgMatches = command.get_matches();
    let mut rlox: rlox::RLox = RLox::new();

    if let Some(source_path) = matches.get_one("source_path") {
        rlox.run_file(source_path);
    } else {
        rlox.run_prompt();
    }
}

fn parse_args() -> Command {
    command!().arg(Arg::new("source_path").id("source_path").required(false))
}
