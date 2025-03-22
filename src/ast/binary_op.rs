use crate::ast::agda_expr::AgdaExpr;
use crate::ast::operator::Operator;

/// A type to denote Binary Operations in Agda.
/// Consists of the operator, and the lhs and rhs of the expression.
pub type BinOperator = (Operator, AgdaExpr, AgdaExpr);