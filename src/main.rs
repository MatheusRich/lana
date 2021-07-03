mod interpreter;
mod lana_err;
mod lexer;
mod parser;
mod repl;

use interpreter::LanaEnv;
use lana_err::LanaErr;
use lexer::Tokenizer;
use lexer::{Token, TokenKind};
use parser::{parse, LanaExpr, LanaLambda};
use repl::repl;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(args[1].clone()),
        _ => {
            print_error(
                "Wrong number of arguments. Use `lana some-file.lana` or just `lana` for REPL",
            );
            std::process::exit(-1);
        }
    }
}

fn run_file(filename: String) {
    use std::fs;

    let input: String;

    match fs::read_to_string(filename) {
        Ok(content) => input = content,
        Err(msg) => {
            print_error(&msg);
            return;
        }
    };

    if let Err(error) = eval(input) {
        print_error(error);
    }
}

fn eval(input: String) -> Result<(), LanaErr> {
    let tokens = lexer::Tokenizer::new(&input).tokens();
    let exprs = parser::parse_all(&tokens)?;

    let mut env = LanaEnv::default();
    for expr in exprs {
        interpreter::eval(&expr, &mut env)?;
    }

    Ok(())
}

fn print_error(msg: impl std::fmt::Display) {
    use colored::Colorize;

    let s = format!("ERROR: {}.", msg).bold().red().to_string();

    println!("{}", s);
}
