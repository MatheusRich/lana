use super::RispErr;
use std::rc::Rc;

#[derive(Clone)]
pub enum RispExpr {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<RispExpr>),
    Func(fn(&[RispExpr]) -> Result<RispExpr, RispErr>),
    Lambda(RispLambda),
}

impl RispExpr {
    pub fn enum_name(&self) -> String {
        match self {
            RispExpr::Bool(_b) => "boolean".into(),
            RispExpr::Symbol(_s) => "symbol".into(),
            RispExpr::Number(_n) => "number".into(),
            RispExpr::List(_) => "list".into(),
            RispExpr::Func(_) => "function".into(),
            RispExpr::Lambda(_) => "lambda".into(),
        }
    }

    pub fn to_colorized_string(&self) -> String {
        use colored::Colorize;

        match self {
            RispExpr::Bool(_) => self.to_string().bold().purple().to_string(),
            RispExpr::Symbol(_) => self.to_string().bold().yellow().to_string(),
            RispExpr::Number(_) => self.to_string().bold().cyan().to_string(),
            RispExpr::List(list) => {
                let xs: Vec<String> = list
                    .iter()
                    .map(|value| value.to_colorized_string())
                    .collect();

                format!("({})", xs.join(", "))
            }
            RispExpr::Func(_) => self.to_string().green().to_string(),
            RispExpr::Lambda(_) => self.to_string().green().to_string(),
        }
    }
}

impl std::fmt::Display for RispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            RispExpr::Bool(boolean) => boolean.to_string(),
            RispExpr::Symbol(s) => s.clone(),
            RispExpr::Number(n) => n.to_string(),
            RispExpr::Func(function) => format!("fn({:p})", function),
            RispExpr::Lambda(lambda) => format!("lambda({:p})", lambda),
            RispExpr::List(list) => {
                let xs: Vec<String> = list.iter().map(|value| value.to_string()).collect();

                format!("({})", xs.join(", "))
            }
        };

        write!(f, "{}", string)
    }
}

impl std::fmt::Debug for RispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} '{}'", self.enum_name(), self.to_string())
    }
}

#[derive(Clone)]
pub struct RispLambda {
    pub params: Rc<RispExpr>,
    pub body: Rc<RispExpr>,
}
