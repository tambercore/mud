use crate::ast::agda_expr::AgdaExpr;
use crate::ast::var_declaration::VarDecl;

/// A type to denote Quantification in Agda.
/// Consists of the quantifier symbol, a list of dependent variables and an expression body.
#[derive(Clone)]
pub struct Quantification {pub symbol : String, pub vars : Vec<VarDecl>, pub expr : Box<AgdaExpr>}

impl PartialEq for Quantification {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.vars == other.vars && self.expr == other.expr
    }
}

#[macro_export]
macro_rules! quant {
    ($symbol:expr, $vars:expr, $expr:expr) => {
        AgdaExpr::Quant(
        Quantification {
            symbol: $symbol.to_string(),
            vars: $vars,
            expr: Box::new($expr)
        }
        )
    };
}