use super::LanaErr;
use std::rc::Rc;

#[derive(Clone)]
pub enum LanaExpr {
    Nil,
    Bool(bool),
    Keyword(String),
    Symbol(String),
    Number(f64),
    List(Vec<LanaExpr>),
    Func(fn(&[LanaExpr]) -> Result<LanaExpr, LanaErr>),
    Lambda(LanaLambda),
}

impl LanaExpr {
    pub fn enum_name(&self) -> String {
        match self {
            LanaExpr::Bool(_b) => "boolean".into(),
            LanaExpr::Symbol(_s) => "symbol".into(),
            LanaExpr::Keyword(_s) => "keyword".into(),
            LanaExpr::Number(_n) => "number".into(),
            LanaExpr::List(_) => "list".into(),
            LanaExpr::Func(_) => "function".into(),
            LanaExpr::Lambda(_) => "lambda".into(),
            LanaExpr::Nil => "nil".into(),
        }
    }

    pub fn to_colorized_string(&self) -> String {
        use colored::Colorize;

        match self {
            LanaExpr::Nil => self.to_string().bold().purple().to_string(),
            LanaExpr::Bool(_) => self.to_string().bold().purple().to_string(),
            LanaExpr::Symbol(_) => self.to_string().bold().yellow().to_string(),
            LanaExpr::Keyword(_) => self.to_string().bold().yellow().to_string(),
            LanaExpr::Number(_) => self.to_string().bold().cyan().to_string(),
            LanaExpr::List(list) => {
                let xs: Vec<String> = list
                    .iter()
                    .map(|value| value.to_colorized_string())
                    .collect();

                format!("({})", xs.join(", "))
            }
            LanaExpr::Func(_) => self.to_string().green().to_string(),
            LanaExpr::Lambda(_) => self.to_string().green().to_string(),
        }
    }
}

impl std::fmt::Display for LanaExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            LanaExpr::Nil => "nil".to_string(),
            LanaExpr::Bool(boolean) => boolean.to_string(),
            LanaExpr::Symbol(s) => s.clone(),
            LanaExpr::Keyword(s) => s.clone(),
            LanaExpr::Number(n) => n.to_string(),
            LanaExpr::Func(function) => format!("fn({})", *function as usize),
            LanaExpr::Lambda(lambda) => format!("lambda({:p})", lambda),
            LanaExpr::List(list) => {
                let xs: Vec<String> = list.iter().map(|value| value.to_string()).collect();

                format!("({})", xs.join(", "))
            }
        };

        write!(f, "{}", string)
    }
}

impl std::fmt::Debug for LanaExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanaExpr::Nil => write!(f, "{}", self.to_string()),
            _ => write!(f, "{} '{}'", self.enum_name(), self.to_string()),
        }
    }
}

impl PartialEq for LanaExpr {
    fn eq(&self, other: &Self) -> bool {
        if self.enum_name() != other.enum_name() {
            return false;
        }

        self.to_string() == other.to_string()
    }
}

#[derive(Clone, PartialEq)]
pub struct LanaLambda {
    pub params: Rc<LanaExpr>,
    pub body: Rc<LanaExpr>,
}
