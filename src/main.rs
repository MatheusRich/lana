mod risp_env;
mod risp_err;
mod risp_expr;

use risp_env::RispEnv;
use risp_err::RispErr;
use risp_expr::{RispExpr, RispLambda};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    repl();
}

fn repl() {
    use colored::Colorize;

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

fn tokenize(code: String) -> Vec<String> {
    code.replace("(", " ( ")
        .replace(")", " ) ")
        .replace(",", " ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}

// Parser

fn parse(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| RispErr::Reason("Could not get token".into()))?;

    match token.as_str() {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("Unexpected ')'".into())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq(tokens: &[String]) -> Result<(RispExpr, &[String]), RispErr> {
    let mut res: Vec<RispExpr> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| RispErr::Reason("could not find closing ')'".into()))?;

        if next_token == ")" {
            return Ok((RispExpr::List(res), rest));
        }

        let (expr, new_xs) = parse(&xs)?;

        res.push(expr);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> RispExpr {
    match token {
        "true" => RispExpr::Bool(true),
        "false" => RispExpr::Bool(false),
        _ => {
            let potential_number = token.parse::<f64>();
            match potential_number {
                Ok(value) => RispExpr::Number(value),
                Err(_) => RispExpr::Symbol(token.to_string()),
            }
        }
    }
}

// Interpreter

fn eval(expr: &RispExpr, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    match expr {
        RispExpr::Bool(_bool) => Ok(expr.clone()),
        RispExpr::Symbol(k) => {
            env_get(k, env).ok_or_else(|| RispErr::Reason(format!("Undefined symbol '{}'", k)))
        }
        RispExpr::Number(_n) => Ok(expr.clone()),
        RispExpr::List(list) => {
            let (first_form, arg_forms) = list
                .split_first()
                .ok_or_else(|| RispErr::Reason("Expected a non-empty list".into()))?;

            match eval_built_in_form(first_form, arg_forms, env) {
                Some(result) => result,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        RispExpr::Func(function) => {
                            let args_eval: Result<Vec<RispExpr>, RispErr> =
                                arg_forms.iter().map(|arg| eval(arg, env)).collect();
                            function(&args_eval?)
                        }
                        RispExpr::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params, arg_forms, env)?;
                            eval(&lambda.body, new_env)
                        }
                        _ => Err(RispErr::Reason(format!(
                            "First form must be a function, got {} '{}'",
                            first_eval.enum_name(),
                            first_eval.to_string()
                        ))),
                    }
                }
            }
        }
        RispExpr::Func(_) => Err(RispErr::Reason("Unexpected function".to_string())),
        RispExpr::Lambda(_) => Err(RispErr::Reason("Unexpected lambda".to_string())),
    }
}

fn env_get(symbol: &str, env: &RispEnv) -> Option<RispExpr> {
    match env.data.get(symbol) {
        Some(expr) => Some(expr.clone()),
        None => match &env.outer {
            Some(outer_env) => env_get(symbol, outer_env),
            None => None,
        },
    }
}

fn eval_exprs(args: &[RispExpr], env: &mut RispEnv) -> Result<Vec<RispExpr>, RispErr> {
    args.iter().map(|arg| eval(arg, env)).collect()
}

fn parse_list_of_symbol_strings(expr: Rc<RispExpr>) -> Result<Vec<String>, RispErr> {
    let list = match expr.as_ref() {
        RispExpr::List(s) => Ok(s.clone()),
        _ => Err(RispErr::Reason("Expected lambda args to be a list".into())),
    }?;

    list.iter()
        .map(|arg| match arg {
            RispExpr::Symbol(s) => Ok(s.clone()),
            _ => Err(RispErr::Reason(
                "Expected symbols in lambda argument list".into(),
            )),
        })
        .collect()
}

fn env_for_lambda<'a>(
    params: Rc<RispExpr>,
    args: &[RispExpr],
    outer_env: &'a mut RispEnv,
) -> Result<RispEnv<'a>, RispErr> {
    let symbols = parse_list_of_symbol_strings(params)?;

    if symbols.len() != args.len() {
        return Err(RispErr::Reason(format!(
            "Expected {} arguments, got {}",
            symbols.len(),
            args.len()
        )));
    }

    let vs = eval_exprs(args, outer_env)?;
    let mut data: HashMap<String, RispExpr> = HashMap::new();

    for (k, v) in symbols.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }

    Ok(RispEnv {
        data,
        outer: Some(outer_env),
    })
}

fn eval_built_in_form(
    expr: &RispExpr,
    args: &[RispExpr],
    env: &mut RispEnv,
) -> Option<Result<RispExpr, RispErr>> {
    match expr {
        RispExpr::Symbol(s) => match s.as_str() {
            "if" => Some(eval_if_args(args, env)),
            "def" => Some(eval_def_args(args, env)),
            "fn" => Some(eval_lambda_args(args)),
            _ => None,
        },
        _ => None,
    }
}

fn eval_if_args(args: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let condition_expr = args
        .first()
        .ok_or_else(|| RispErr::Reason("Expected if condition".into()))?;

    let condition_eval = eval(condition_expr, env)?;

    match condition_eval {
        RispExpr::Bool(boolean) => {
            let branch_index = if boolean { 1 } else { 2 };
            let branch_name = if boolean { "then" } else { "else" };

            let if_branch = args
                .get(branch_index)
                .ok_or_else(|| RispErr::Reason(format!("Expected if's {} branch", branch_name)))?;

            eval(if_branch, env)
        }
        _ => Err(RispErr::Reason(format!(
            "Expected boolean in if condition, got {:?}",
            condition_eval,
        ))),
    }
}

fn eval_def_args(args: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let variable = args
        .first()
        .ok_or_else(|| RispErr::Reason("Expected variable name".into()))?;

    let var_name = match variable {
        RispExpr::Symbol(name) => Ok(name.clone()),
        _ => Err(RispErr::Reason(format!(
            "Expected variable name to be a symbol, got {:?}",
            variable
        ))),
    }?;

    if args.len() > 2 {
        return Err(RispErr::Reason(format!(
            "Expected only two arguments in assignment, got {}",
            args.len()
        )));
    }
    let value_expr = args
        .get(1)
        .ok_or_else(|| RispErr::Reason("Expected assignment value".into()))?;

    let value = eval(value_expr, env)?;
    env.data.insert(var_name, value.clone());

    Ok(value)
}

fn eval_lambda_args(args: &[RispExpr]) -> Result<RispExpr, RispErr> {
    let params = args
        .first()
        .ok_or_else(|| RispErr::Reason("Expected lambda args and body".into()))?;
    let body = args
        .get(1)
        .ok_or_else(|| RispErr::Reason("Expected lambda body".into()))?;

    if args.len() > 2 {
        return Err(RispErr::Reason(
            "Lambdas definition takes only 2 arguments".into(),
        ));
    }

    Ok(RispExpr::Lambda(RispLambda {
        body: Rc::new(body.clone()),
        params: Rc::new(params.clone()),
    }))
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tokenizes_input() {
        assert_eq!(
            vec!["(", "+", "10", "5", ")"],
            tokenize("(+ 10 5)".to_string())
        );
    }
}
