#![allow(warnings, dead_code)]

use clap::{command, Arg, ArgMatches, Command};

mod rlox;
mod token_type;
mod token;
mod object;
mod scanner;
mod expression;
mod parser;

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
