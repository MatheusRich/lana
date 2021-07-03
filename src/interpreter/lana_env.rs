// use super::{prelude, LanaExpr};
use super::prelude;
use super::LanaExpr;
use std::collections::HashMap;

type EnvData = HashMap<String, LanaExpr>;

pub struct LanaEnv<'a> {
    pub data: EnvData,
    pub outer: Option<&'a LanaEnv<'a>>,
}

impl<'a> LanaEnv<'a> {
    pub fn default() -> Self {
        LanaEnv {
            data: prelude::prelude(),
            outer: None,
        }
    }

    pub fn get(&self, symbol: &str) -> Option<LanaExpr> {
        match self.data.get(symbol) {
            Some(expr) => Some(expr.clone()),
            None => match &self.outer {
                Some(outer_env) => outer_env.get(symbol),
                None => None,
            },
        }
    }
}
