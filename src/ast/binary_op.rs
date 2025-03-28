use crate::ast::agda_expr::AgdaExpr;
use crate::ast::operator::Operator;

/// A type to denote Binary Operations in Agda.
/// Consists of the operator, and the lhs and rhs of the expression.
#[derive(Eq, Hash, Debug, Clone)]
pub struct BinOperator {pub symbol : Operator, pub lhs : Box<AgdaExpr>, pub rhs : Box<AgdaExpr>}

impl PartialEq for BinOperator {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.lhs == other.lhs && self.rhs == other.rhs
    }
}

#[macro_export]
macro_rules! bin_op {
    ($lhs:expr, $rhs:expr, $op:expr) => {
        AgdaExpr::BinOp( BinOperator {
            symbol: $op,
            lhs: Box::new($lhs),
            rhs: Box::new($rhs),
        })
    };
}