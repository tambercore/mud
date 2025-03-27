use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Abstractions in Agda.
/// These are of the form λ x → y, where x is an identifier and y is an expression.
#[derive(Debug, Clone)]
pub struct Abstraction
{pub var: String, pub expr: Box<AgdaExpr>}

impl PartialEq for Abstraction {
    fn eq(&self, other: &Self) -> bool {
        self.var == other.var && self.expr == other.expr
    }
}

#[macro_export]
macro_rules! abstraction {
    ($var:expr, $expr:expr) => {
        AgdaExpr::Abs( Abstraction { var: $var.to_string(), expr: Box::new($expr) })
    };
}