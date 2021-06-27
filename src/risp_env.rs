use super::{RispErr, RispExpr};
use std::collections::HashMap;

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

pub struct RispEnv<'a> {
    pub data: HashMap<String, RispExpr>,
    pub outer: Option<&'a RispEnv<'a>>,
}

impl<'a> RispEnv<'a> {
    pub fn default() -> Self {
        let mut std_lib: HashMap<String, RispExpr> = HashMap::new();

        std_lib.insert(
            "+".to_string(),
            RispExpr::Func(|args| {
                let sum = parse_list_of_floats(args)?.iter().sum();

                Ok(RispExpr::Number(sum))
            }),
        );

        std_lib.insert(
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

        std_lib.insert(
            "*".to_string(),
            RispExpr::Func(|args| {
                let result = parse_list_of_floats(args)?
                    .iter()
                    .fold(1.0, |res, n| res * n);

                Ok(RispExpr::Number(result))
            }),
        );

        std_lib.insert(
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

        std_lib.insert(
            "=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| f64_aprox_eq(a, b))),
        );

        std_lib.insert(
            ">".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a > b)),
        );
        std_lib.insert(
            ">=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a >= b)),
        );
        std_lib.insert(
            "<".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a < b)),
        );
        std_lib.insert(
            "<=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a <= b)),
        );

        std_lib.insert(
            "<=".to_string(),
            RispExpr::Func(ensure_tonicity!(|a, b| a <= b)),
        );

        std_lib.insert(
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

        std_lib.insert(
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

        std_lib.insert(
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

        RispEnv {
            data: std_lib,
            outer: None,
        }
    }
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
