mod interpreter;
mod lana_env;
mod lana_err;
mod lana_expr;
mod lexer;
mod parser;
mod prelude;
mod repl;

use lana_env::LanaEnv;
use lana_err::LanaErr;
use lana_expr::{LanaExpr, LanaLambda};
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
        match error {
            LanaErr::Reason(msg) => {
                print_error(&msg);
            }
        }
    }
}

fn eval(input: String) -> Result<(), LanaErr> {
    let exprs = parser::parse_all(&lexer::tokenize(input))?;

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
