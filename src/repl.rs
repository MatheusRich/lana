use super::interpreter::eval;
use super::parser::parse;
use super::risp_env::RispEnv;
use super::risp_err::RispErr;
use super::risp_expr::RispExpr;
use super::tokenize::tokenize;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn repl() {
    let env = &mut RispEnv::default();
    let mut rl = Editor::<()>::new();

    loop {
        let input = match rl.readline("risp> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                line.trim().to_string()
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Unexpected error while reading input: {:?}", err);
                break;
            }
        };

        if input == "quit" || input == "exit" {
            break;
        }

        if input == "help" {
            println!("Sorry, the author was too lazy to actually code this 😅.");
            continue;
        }

        if input.is_empty() {
            continue;
        }

        match parse_eval(input, env) {
            Ok(res) => {
                env.data.insert("_".into(), res.clone());

                println!("=> {}", res.to_colorized_string())
            }
            Err(e) => match e {
                RispErr::Reason(msg) => {
                    let s = format!("ERROR: {}.", msg).bold().red().to_string();

                    println!("=> {}", s)
                }
            },
        }
    }
}

fn parse_eval(expr: String, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let (parsed_expr, _) = parse(&tokenize(expr))?;
    let evaled_expr = eval(&parsed_expr, env)?;

    Ok(evaled_expr)
}
