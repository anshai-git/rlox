#![allow(warnings, dead_code)]

use clap::{command, Arg, ArgMatches, Command};

mod ast_printer;
mod environment;
mod expr;
mod interpreter;
mod literal; // deprecated, instead: Object
mod lox_callable;
mod native_functions;
mod object;
mod parser;
mod rlox;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod lox_function;

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
