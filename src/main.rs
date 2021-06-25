mod interpreter;
mod parser;
mod repl;
mod risp_env;
mod risp_err;
mod risp_expr;
mod tokenize;

use repl::repl;
use risp_env::RispEnv;
use risp_err::RispErr;
use risp_expr::{RispExpr, RispLambda};

fn main() {
    repl();
}
