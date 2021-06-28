use super::{RispErr, RispExpr};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[RispExpr]| -> Result<RispExpr, RispErr> {
            let floats = parse_list_of_floats(args)?;
            let first: &f64 = floats
                .first()
                .ok_or_else(|| RispErr::Reason("Expected at least one number".to_string()))?;
            let rest = &floats[1..];

            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            }

            Ok(RispExpr::Bool(f(first, rest)))
        }
    }};
}

pub fn prelude() -> HashMap<String, RispExpr> {
    let mut prelude: HashMap<String, RispExpr> = HashMap::new();

    prelude.insert(
        "+".to_string(),
        RispExpr::Func(|args| {
            let sum = parse_list_of_floats(args)?.iter().sum();

            Ok(RispExpr::Number(sum))
        }),
    );

    prelude.insert(
        "-".to_string(),
        RispExpr::Func(|args| {
            let numbers = parse_list_of_floats(args)?;
            let (first, rest) = numbers
                .split_first()
                .ok_or_else(|| RispErr::Reason("Expected at least one number".into()))?;
            let sum_of_rest: f64 = rest.iter().sum();

            Ok(RispExpr::Number(first - sum_of_rest))
        }),
    );

    prelude.insert(
        "*".to_string(),
        RispExpr::Func(|args| {
            let result = parse_list_of_floats(args)?
                .iter()
                .fold(1.0, |res, n| res * n);

            Ok(RispExpr::Number(result))
        }),
    );

    prelude.insert(
        "/".to_string(),
        RispExpr::Func(|args| {
            let numbers = parse_list_of_floats(args)?;
            let (first, rest) = numbers
                .split_first()
                .ok_or_else(|| RispErr::Reason("Expected at least one number".into()))?;
            let product_of_rest: f64 = rest.iter().fold(1.0, |res, n| res * n);

            Ok(RispExpr::Number(first / product_of_rest))
        }),
    );

    prelude.insert(
        "=".to_string(),
        RispExpr::Func(ensure_tonicity!(|a, b| f64_aprox_eq(a, b))),
    );

    prelude.insert(
        ">".to_string(),
        RispExpr::Func(ensure_tonicity!(|a, b| a > b)),
    );
    prelude.insert(
        ">=".to_string(),
        RispExpr::Func(ensure_tonicity!(|a, b| a >= b)),
    );
    prelude.insert(
        "<".to_string(),
        RispExpr::Func(ensure_tonicity!(|a, b| a < b)),
    );
    prelude.insert(
        "<=".to_string(),
        RispExpr::Func(ensure_tonicity!(|a, b| a <= b)),
    );

    prelude.insert(
        "<=".to_string(),
        RispExpr::Func(ensure_tonicity!(|a, b| a <= b)),
    );

    prelude.insert(
        "println".to_string(),
        RispExpr::Func(|args| {
            if args.is_empty() {
                return Err(RispErr::Reason("Expected at least one argument".into()));
            }

            for arg in args {
                println!("{}", arg);
            }

            Ok(args[0].clone())
        }),
    );

    prelude.insert(
        "print".to_string(),
        RispExpr::Func(|args| {
            if args.is_empty() {
                return Err(RispErr::Reason("Expected at least one argument".into()));
            }

            for arg in args {
                print!("{}", arg);
            }

            Ok(args[0].clone())
        }),
    );

    prelude.insert(
        "gets".to_string(),
        RispExpr::Func(|args| {
            if !args.is_empty() {
                return Err(RispErr::Reason(format!(
                    "Expected no arguments, got {}",
                    args.len()
                )));
            }

            let mut s = String::new();

            std::io::stdin()
                .read_line(&mut s)
                .map(|_| s = s.trim().to_string())
                .map_err(|_| RispErr::Reason("Failed to read line".into()))?;

            match s.parse::<f64>() {
                Ok(n) => Ok(RispExpr::Number(n)),
                Err(_) => Err(RispErr::Reason("Could not parse number".into())),
            }
        }),
    );

    prelude.insert(
        "sleep".to_string(),
        RispExpr::Func(|args| {
            if args.len() != 1 {
                return Err(RispErr::Reason(format!(
                    "Expected 1 argument, got {}",
                    args.len()
                )));
            }

            match &args[0] {
                RispExpr::Number(time) => {
                    let seconds = *time as u64;
                    sleep(Duration::new(seconds, 0));

                    Ok(RispExpr::Number(seconds as f64))
                }
                expr => Err(RispErr::Reason(format!(
                    "Invalid argument: expected number, got {:?}",
                    expr
                ))),
            }
        }),
    );

    prelude
}

fn f64_aprox_eq(a: &f64, b: &f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

fn parse_list_of_floats(list: &[RispExpr]) -> Result<Vec<f64>, RispErr> {
    list.iter().map(|n| parse_single_float(n)).collect()
}

fn parse_single_float(expr: &RispExpr) -> Result<f64, RispErr> {
    match expr {
        RispExpr::Number(n) => Ok(*n),
        other_expr => Err(RispErr::Reason(format!(
            "Expected a number, got {:?}",
            other_expr
        ))),
    }
}
