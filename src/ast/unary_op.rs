use crate::ast::agda_expr::AgdaExpr;
use crate::ast::operator::Operator;

/// A type to denote Unary Operators in Agda.
/// Consists of the Operator and the expression which is being applied.
pub struct UnOperator {pub op : Operator, pub expr : Box<AgdaExpr>}

impl PartialEq for UnOperator {
    fn eq(&self, other: &Self) -> bool {
        if self.op == other.op {
            self.expr == other.expr
        } else {
            false
        }
    }
}
#[macro_export]
macro_rules! term {
    ($e:expr) => {
        Box::from(AgdaExpr::Term($e))
    };
}