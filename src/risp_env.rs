use super::{prelude, RispExpr};
use std::collections::HashMap;

type EnvData = HashMap<String, RispExpr>;

pub struct RispEnv<'a> {
    pub data: EnvData,
    pub outer: Option<&'a RispEnv<'a>>,
}

impl<'a> RispEnv<'a> {
    pub fn default() -> Self {
        RispEnv {
            data: prelude::prelude(),
            outer: None,
        }
    }

    pub fn get(&self, symbol: &str) -> Option<RispExpr> {
        match self.data.get(symbol) {
            Some(expr) => Some(expr.clone()),
            None => match &self.outer {
                Some(outer_env) => outer_env.get(symbol),
                None => None,
            },
        }
    }
}
