mod risp_env;
mod risp_err;
mod risp_expr;

use risp_env::RispEnv;
use risp_err::RispErr;
use risp_expr::RispExpr;

fn main() {
    let env = &mut RispEnv::default();

    loop {
        print!("risp >  ");
        let expr = slurp_expr();

        match parse_eval(expr, env) {
            Ok(res) => println!("🔥 => {}", res),
            Err(e) => match e {
                RispErr::Reason(msg) => println!("🙀 => {}", msg),
            },
        }
    }
}

fn parse_eval(expr: String, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let (parsed_expr, _) = parse(&tokenize(expr))?;
    let evaled_expr = eval(&parsed_expr, env)?;

    Ok(evaled_expr)
}

fn slurp_expr() -> String {
    use std::io::Write;

    let mut expr = String::new();

    std::io::stdout().flush().expect("could not flush stdout");
    std::io::stdin()
        .read_line(&mut expr)
        .expect("failed to read_line");
    expr
}

fn tokenize(code: String) -> Vec<String> {
    code.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}

// Parser

fn parse<'a>(tokens: &'a [String]) -> Result<(RispExpr, &'a [String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| RispErr::Reason("could not get token".into()))?;

    match token.as_str() {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("expected ')'".into())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(RispExpr, &'a [String]), RispErr> {
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
    let potential_number = token.parse::<f64>();

    match potential_number {
        Ok(value) => RispExpr::Number(value),
        Err(_) => RispExpr::Symbol(token.to_string()),
    }
}

// Interpreter

fn eval(expr: &RispExpr, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    match expr {
        RispExpr::Symbol(k) => env
            .data
            .get(k)
            .ok_or_else(|| RispErr::Reason(format!("unexpected symbol k='{}'", k)))
            .map(|var| var.clone()),
        RispExpr::Number(_n) => Ok(expr.clone()),
        RispExpr::List(list) => {
            let (first_form, arg_forms) = list
                .split_first()
                .ok_or_else(|| RispErr::Reason("expected a non-empty list".into()))?;
            let first_eval = eval(first_form, env)?;

            match first_eval {
                RispExpr::Func(function) => {
                    let args_eval: Result<Vec<RispExpr>, RispErr> =
                        arg_forms.iter().map(|arg| eval(arg, env)).collect();

                    function(&args_eval?)
                }
                _ => Err(RispErr::Reason(format!(
                    "first form must be a function. Got {} '{}'",
                    first_eval.enum_name(),
                    first_eval.name()
                ))),
            }
        }
        _ => Err(RispErr::Reason("".into())),
    }
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
