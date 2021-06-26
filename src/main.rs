mod interpreter;
mod parser;
mod repl;
mod risp_env;
mod risp_err;
mod risp_expr;
mod tokenize;

use repl::repl;
use risp_env::RispEnv;
use risp_err::RispErr;
use risp_expr::{RispExpr, RispLambda};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(args[1].clone()),
        _ => {
            eprintln!("Wrong number of arguments");
            std::process::exit(-1);
        }
    }
}

fn run_file(filename: String) {
    use colored::Colorize;
    use std::fs;

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file!");

    if let Err(error) = eval(input) {
        match error {
            RispErr::Reason(msg) => {
                let s = format!("ERROR: {}.", msg).bold().red().to_string();

                println!("{}", s)
            }
        }
    }
}

fn eval(input: String) -> Result<(), RispErr> {
    let exprs = parser::parse_all(&tokenize::tokenize(input))?;

    let mut env = RispEnv::default();
    for expr in exprs {
        interpreter::eval(&expr, &mut env)?;
    }

    Ok(())
}
