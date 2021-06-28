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
}
