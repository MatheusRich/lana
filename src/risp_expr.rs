use super::RispErr;

#[derive(Clone)]
pub enum RispExpr {
    Symbol(String),
    Number(f64),
    List(Vec<RispExpr>),
    Func(fn(&[RispExpr]) -> Result<RispExpr, RispErr>),
}

impl RispExpr {
    pub fn enum_name(&self) -> String {
        match self {
            RispExpr::Symbol(_s) => "symbol".into(),
            RispExpr::Number(_n) => "number".into(),
            RispExpr::List(_) => "list".into(),
            RispExpr::Func(_) => "func".into(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            RispExpr::Symbol(s) => s.clone(),
            RispExpr::Number(n) => n.to_string(),
            RispExpr::List(_) => "list".into(),
            RispExpr::Func(_) => "function".into(),
        }
    }
}

impl std::fmt::Display for RispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            RispExpr::Symbol(s) => s.clone(),
            RispExpr::Number(n) => n.to_string(),
            RispExpr::Func(_fn) => "Function {}".to_string(),
            RispExpr::List(list) => {
                let xs: Vec<String> = list.iter().map(|value| value.to_string()).collect();

                format!("({})", xs.join(", "))
            }
        };

        write!(f, "{}", string)
    }
}
