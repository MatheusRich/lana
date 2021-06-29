use super::{LanaEnv, LanaErr, LanaExpr, LanaLambda};

use std::collections::HashMap;
use std::rc::Rc;

pub fn eval(expr: &LanaExpr, env: &mut LanaEnv) -> Result<LanaExpr, LanaErr> {
    match expr {
        LanaExpr::Nil => Ok(LanaExpr::Nil),
        LanaExpr::Bool(_) => Ok(expr.clone()),
        LanaExpr::Keyword(_) => Ok(expr.clone()),
        LanaExpr::Number(_) => Ok(expr.clone()),
        LanaExpr::Symbol(k) => env
            .get(k)
            .ok_or_else(|| LanaErr::Reason(format!("Undefined symbol '{}'", k))),
        LanaExpr::List(list) => {
            let (first_form, arg_forms) = list
                .split_first()
                .ok_or_else(|| LanaErr::Reason("Expected a non-empty list".into()))?;

            match eval_built_in_form(first_form, arg_forms, env) {
                Some(result) => result,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        LanaExpr::Func(function) => {
                            let args_eval: Result<Vec<LanaExpr>, LanaErr> =
                                arg_forms.iter().map(|arg| eval(arg, env)).collect();
                            function(&args_eval?)
                        }
                        LanaExpr::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params, arg_forms, env)?;
                            eval(&lambda.body, new_env)
                        }
                        _ => Err(LanaErr::Reason(format!(
                            "First form must be a function, got {:?}",
                            first_eval
                        ))),
                    }
                }
            }
        }
        LanaExpr::Func(_) => Err(LanaErr::Reason("Unexpected function".to_string())),
        LanaExpr::Lambda(_) => Err(LanaErr::Reason("Unexpected lambda".to_string())),
    }
}

fn eval_exprs(args: &[LanaExpr], env: &mut LanaEnv) -> Result<Vec<LanaExpr>, LanaErr> {
    args.iter().map(|arg| eval(arg, env)).collect()
}

fn parse_list_of_symbol_strings(expr: Rc<LanaExpr>) -> Result<Vec<String>, LanaErr> {
    let list = match expr.as_ref() {
        LanaExpr::List(s) => Ok(s.clone()),
        _ => Err(LanaErr::Reason("Expected lambda args to be a list".into())),
    }?;

    list.iter()
        .map(|arg| match arg {
            LanaExpr::Symbol(s) => Ok(s.clone()),
            _ => Err(LanaErr::Reason(
                "Expected symbols in lambda argument list".into(),
            )),
        })
        .collect()
}

fn env_for_lambda<'a>(
    params: Rc<LanaExpr>,
    args: &[LanaExpr],
    outer_env: &'a mut LanaEnv,
) -> Result<LanaEnv<'a>, LanaErr> {
    let symbols = parse_list_of_symbol_strings(params)?;

    if symbols.len() != args.len() {
        return Err(LanaErr::Reason(format!(
            "Expected {} argument(s), got {}",
            symbols.len(),
            args.len()
        )));
    }

    let vs = eval_exprs(args, outer_env)?;
    let mut data: HashMap<String, LanaExpr> = HashMap::new();

    for (k, v) in symbols.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }

    Ok(LanaEnv {
        data,
        outer: Some(outer_env),
    })
}

fn eval_built_in_form(
    expr: &LanaExpr,
    args: &[LanaExpr],
    env: &mut LanaEnv,
) -> Option<Result<LanaExpr, LanaErr>> {
    match expr {
        LanaExpr::Symbol(s) => match s.as_str() {
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

fn eval_if_args(args: &[LanaExpr], env: &mut LanaEnv) -> Result<LanaExpr, LanaErr> {
    let condition_expr = args
        .first()
        .ok_or_else(|| LanaErr::Reason("Expected if condition".into()))?;

    if args.len() > 3 {
        return Err(LanaErr::Reason(format!(
            "Expected 2-3 arguments, got {}",
            args.len()
        )));
    }

    let branch_name = match eval(condition_expr, env)? {
        LanaExpr::Bool(false) | LanaExpr::Nil => "else",
        _ => "then",
    };

    let branch_index = if branch_name == "then" { 1 } else { 2 };

    let if_branch = args
        .get(branch_index)
        .ok_or_else(|| LanaErr::Reason(format!("Expected if's {} branch", branch_name)))?;

    eval(if_branch, env)
}

fn eval_def_args(args: &[LanaExpr], env: &mut LanaEnv) -> Result<LanaExpr, LanaErr> {
    let variable = args
        .first()
        .ok_or_else(|| LanaErr::Reason("Expected variable name".into()))?;

    let var_name = match variable {
        LanaExpr::Symbol(name) => Ok(name.clone()),
        _ => Err(LanaErr::Reason(format!(
            "Expected variable name to be a symbol, got {:?}",
            variable
        ))),
    }?;

    if args.len() > 2 {
        return Err(LanaErr::Reason(format!(
            "Expected only two arguments in assignment, got {}",
            args.len()
        )));
    }

    let value_expr = args
        .get(1)
        .ok_or_else(|| LanaErr::Reason("Expected assignment value".into()))?;

    let value = eval(value_expr, env)?;
    env.data.insert(var_name, value.clone());

    Ok(value)
}

fn eval_lambda_args(args: &[LanaExpr]) -> Result<LanaExpr, LanaErr> {
    let params = args
        .first()
        .ok_or_else(|| LanaErr::Reason("Expected lambda args and body".into()))?;
    let body = args
        .get(1)
        .ok_or_else(|| LanaErr::Reason("Expected lambda body".into()))?;

    if args.len() > 2 {
        return Err(LanaErr::Reason(
            "Lambdas definition takes only 2 arguments (args and body)".into(),
        ));
    }

    Ok(LanaExpr::Lambda(LanaLambda {
        body: Rc::new(body.clone()),
        params: Rc::new(params.clone()),
    }))
}

fn eval_defn_args(args: &[LanaExpr], env: &mut LanaEnv) -> Result<LanaExpr, LanaErr> {
    let (variable, lambda_args) = args
        .split_first()
        .ok_or_else(|| LanaErr::Reason("Expected lambda name".into()))?;

    let var_name = match variable {
        LanaExpr::Symbol(name) => Ok(name.clone()),
        _ => Err(LanaErr::Reason(format!(
            "Expected variable name to be a symbol, got {:?}",
            variable
        ))),
    }?;

    let lambda = eval_lambda_args(lambda_args)?;

    env.data.insert(var_name, lambda.clone());

    Ok(lambda)
}

fn eval_do_args(args: &[LanaExpr], env: &mut LanaEnv) -> Result<LanaExpr, LanaErr> {
    if args.is_empty() {
        return Ok(LanaExpr::Nil);
    }

    let mut result = LanaExpr::Number(0.0);

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
        let expr = LanaExpr::List(vec![LanaExpr::Symbol("do".into())]);
        let mut env = LanaEnv::default();

        let result = eval(&expr, &mut env);

        assert_eq!(Ok(LanaExpr::Nil), result)
    }

    #[test]
    fn it_expect_do_macro_to_eval_multiple_exprs() {
        let expr = LanaExpr::List(vec![
            LanaExpr::Symbol("do".into()),
            LanaExpr::List(vec![
                LanaExpr::Symbol("def".into()),
                LanaExpr::Symbol("var1".into()),
                LanaExpr::Number(1.0),
            ]),
            LanaExpr::List(vec![
                LanaExpr::Symbol("def".into()),
                LanaExpr::Symbol("var2".into()),
                LanaExpr::Number(2.0),
            ]),
        ]);
        let mut env = LanaEnv::default();
        env.data.insert("var1".into(), LanaExpr::Number(0.0));
        env.data.insert("var2".into(), LanaExpr::Number(0.0));

        eval(&expr, &mut env).ok();

        assert_eq!(LanaExpr::Number(1.0), env.data.get("var1").unwrap().clone());
        assert_eq!(LanaExpr::Number(2.0), env.data.get("var2").unwrap().clone());
    }

    #[test]
    fn it_expect_do_macro_to_return_last_eval() {
        let expr = LanaExpr::List(vec![
            LanaExpr::Symbol("do".into()),
            LanaExpr::Bool(true),
            LanaExpr::Bool(false),
        ]);
        let mut env = LanaEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval do macro");

        assert_eq!(LanaExpr::Bool(false), result);
    }

    #[test]
    fn it_expect_nil_to_be_falsey() {
        let expr = LanaExpr::List(vec![
            LanaExpr::Symbol("if".into()),
            LanaExpr::Nil,
            LanaExpr::Number(1.0),
            LanaExpr::Number(2.0),
        ]);
        let mut env = LanaEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(LanaExpr::Number(2.0), result);
    }

    #[test]
    fn it_expect_false_to_be_falsey() {
        let expr = LanaExpr::List(vec![
            LanaExpr::Symbol("if".into()),
            LanaExpr::Bool(false),
            LanaExpr::Number(1.0),
            LanaExpr::Number(2.0),
        ]);
        let mut env = LanaEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(LanaExpr::Number(2.0), result);
    }

    #[test]
    fn it_expect_true_to_be_truthy() {
        let expr = LanaExpr::List(vec![
            LanaExpr::Symbol("if".into()),
            LanaExpr::Bool(true),
            LanaExpr::Number(1.0),
            LanaExpr::Number(2.0),
        ]);
        let mut env = LanaEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(LanaExpr::Number(1.0), result);
    }

    #[test]
    fn it_expect_numbers_to_be_truthy() {
        let expr = LanaExpr::List(vec![
            LanaExpr::Symbol("if".into()),
            LanaExpr::Number(0.0),
            LanaExpr::Number(1.0),
            LanaExpr::Number(2.0),
        ]);
        let mut env = LanaEnv::default();

        let result = eval(&expr, &mut env).expect("Could not eval if macro");

        assert_eq!(LanaExpr::Number(1.0), result);
    }
}
