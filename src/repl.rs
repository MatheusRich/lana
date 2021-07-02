use super::interpreter::eval;
use super::lana_env::LanaEnv;
use super::lana_err::LanaErr;
use super::lana_expr::LanaExpr;
use super::lexer::Tokenizer;
use super::parser::parse;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn repl() {
    let env = &mut LanaEnv::default();
    let mut rl = Editor::<()>::new();

    loop {
        let input = match rl.readline(&prompt()) {
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
            Err(e) => {
                let s = format!("ERROR: {}.", e).bold().red().to_string();

                println!("=> {}", s)
            }
        }
    }
}

fn parse_eval(expr: String, env: &mut LanaEnv) -> Result<LanaExpr, LanaErr> {
    let tokens = Tokenizer::new(&expr).tokens();
    let (parsed_expr, _) = parse(&tokens)?;
    let evaled_expr = eval(&parsed_expr, env)?;

    Ok(evaled_expr)
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn prompt() -> String {
    format!("lana v{}> ", VERSION)
}
