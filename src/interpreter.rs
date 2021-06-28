use super::{RispEnv, RispErr, RispExpr, RispLambda};

use std::collections::HashMap;
use std::rc::Rc;

pub fn eval(expr: &RispExpr, env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    match expr {
        RispExpr::Nil => Ok(RispExpr::Nil),
        RispExpr::Bool(_) => Ok(expr.clone()),
        RispExpr::Symbol(k) => {
            env_get(k, env).ok_or_else(|| RispErr::Reason(format!("Undefined symbol '{}'", k)))
        }
        RispExpr::Keyword(_) => Ok(expr.clone()),
        RispExpr::Number(_) => Ok(expr.clone()),
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
                            "First form must be a function, got {:?}",
                            first_eval
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
            "defn" => Some(eval_defn_args(args, env)),
            "do" => Some(eval_do_args(args, env)),
            _ => None,
        },
        _ => None,
    }
}

fn eval_if_args(args: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let condition_expr = args
        .first()
        .ok_or_else(|| RispErr::Reason("Expected if condition".into()))?;

    let branch_name = match eval(condition_expr, env)? {
        RispExpr::Bool(false) | RispExpr::Nil => "else",
        _ => "then",
    };

    let branch_index = if branch_name == "then" { 1 } else { 2 };

    let if_branch = args
        .get(branch_index)
        .ok_or_else(|| RispErr::Reason(format!("Expected if's {} branch", branch_name)))?;

    eval(if_branch, env)
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
            "Lambdas definition takes only 2 arguments (args and body)".into(),
        ));
    }

    Ok(RispExpr::Lambda(RispLambda {
        body: Rc::new(body.clone()),
        params: Rc::new(params.clone()),
    }))
}

fn eval_defn_args(args: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    let (variable, lambda_args) = args
        .split_first()
        .ok_or_else(|| RispErr::Reason("Expected lambda name".into()))?;

    let var_name = match variable {
        RispExpr::Symbol(name) => Ok(name.clone()),
        _ => Err(RispErr::Reason(format!(
            "Expected variable name to be a symbol, got {:?}",
            variable
        ))),
    }?;

    let lambda = eval_lambda_args(lambda_args)?;

    env.data.insert(var_name, lambda.clone());

    Ok(lambda)
}

fn eval_do_args(args: &[RispExpr], env: &mut RispEnv) -> Result<RispExpr, RispErr> {
    if args.is_empty() {
        return Ok(RispExpr::Nil);
    }

    let mut result = RispExpr::Number(0.0);

    for expr in args {
        result = eval(expr, env)?;
    }

    Ok(result.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_expect_macro_do_to_return_nil_if_no_args_are_given() {
        let expr = RispExpr::List(vec![RispExpr::Symbol("do".into())]);
        let mut env = RispEnv::default();

        let result = eval(&expr, &mut env);

        assert_eq!(Ok(RispExpr::Nil), result)
    }

    #[test]
    fn it_expect_do_macro_to_eval_multiple_exprs() {
        let expr = RispExpr::List(vec![
            RispExpr::Symbol("do".into()),
            RispExpr::List(vec![
                RispExpr::Symbol("def".into()),
                RispExpr::Symbol("var1".into()),
                RispExpr::Number(1.0),
            ]),
            RispExpr::List(vec![
                RispExpr::Symbol("def".into()),
                RispExpr::Symbol("var2".into()),
                RispExpr::Number(2.0),
            ]),
        ]);
        let mut env = RispEnv::default();
        env.data.insert("var1".into(), RispExpr::Number(0.0));
        env.data.insert("var2".into(), RispExpr::Number(0.0));

        eval(&expr, &mut env).ok();

        assert_eq!(RispExpr::Number(1.0), env.data.get("var1").unwrap().clone());
        assert_eq!(RispExpr::Number(2.0), env.data.get("var2").unwrap().clone());
    }

    #[test]
    fn it_expect_do_macro_to_return_last_eval() {
        let expr = RispExpr::List(vec![
            RispExpr::Symbol("do".into()),
            RispExpr::Bool(true),
            RispExpr::Bool(false),
        ]);
        let mut env = RispEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval do macro");

        assert_eq!(RispExpr::Bool(false), result);
    }

    #[test]
    fn it_expect_nil_to_be_falsey() {
        let expr = RispExpr::List(vec![
            RispExpr::Symbol("if".into()),
            RispExpr::Nil,
            RispExpr::Number(1.0),
            RispExpr::Number(2.0),
        ]);
        let mut env = RispEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(RispExpr::Number(2.0), result);
    }

    #[test]
    fn it_expect_false_to_be_falsey() {
        let expr = RispExpr::List(vec![
            RispExpr::Symbol("if".into()),
            RispExpr::Bool(false),
            RispExpr::Number(1.0),
            RispExpr::Number(2.0),
        ]);
        let mut env = RispEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(RispExpr::Number(2.0), result);
    }

    #[test]
    fn it_expect_true_to_be_truthy() {
        let expr = RispExpr::List(vec![
            RispExpr::Symbol("if".into()),
            RispExpr::Bool(true),
            RispExpr::Number(1.0),
            RispExpr::Number(2.0),
        ]);
        let mut env = RispEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(RispExpr::Number(1.0), result);
    }

    #[test]
    fn it_expect_numbers_to_be_truthy() {
        let expr = RispExpr::List(vec![
            RispExpr::Symbol("if".into()),
            RispExpr::Number(0.0),
            RispExpr::Number(1.0),
            RispExpr::Number(2.0),
        ]);
        let mut env = RispEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(RispExpr::Number(1.0), result);
    }
}
