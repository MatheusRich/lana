mod interpreter;
mod lana_env;
mod lana_err;
mod lana_expr;
mod parser;
mod prelude;
mod repl;
mod risp_env;
mod risp_err;
mod risp_expr;
mod tokenize;

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
            LanaErr::Reason(msg) => {
                let s = format!("ERROR: {}.", msg).bold().red().to_string();

                println!("{}", s);
            }
        }
    }
}

fn eval(input: String) -> Result<(), LanaErr> {
    let exprs = parser::parse_all(&tokenize::tokenize(input))?;

    let mut env = LanaEnv::default();
    for expr in exprs {
        interpreter::eval(&expr, &mut env)?;
    }

    Ok(())
}
