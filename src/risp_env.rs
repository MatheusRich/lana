use super::{RispErr, RispExpr};
use std::collections::HashMap;

pub struct RispEnv {
    pub data: HashMap<String, RispExpr>,
}

impl RispEnv {
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
                    .ok_or_else(|| RispErr::Reason("expected at least one number".into()))?;
                let sum_of_rest: f64 = rest.iter().sum();

                Ok(RispExpr::Number(first - sum_of_rest))
            }),
        );

        RispEnv { data: std_lib }
    }
}

fn parse_list_of_floats(list: &[RispExpr]) -> Result<Vec<f64>, RispErr> {
    list.iter().map(|n| parse_single_float(n)).collect()
}

fn parse_single_float(expr: &RispExpr) -> Result<f64, RispErr> {
    match expr {
        RispExpr::Number(n) => Ok(*n),
        other => Err(RispErr::Reason(format!(
            "expected a number, got {}",
            other.name()
        ))),
    }
}
