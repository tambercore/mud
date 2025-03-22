use crate::ast::agda_expr::AgdaExpr;
use crate::ast::operator::Operator;

/// A type to denote Binary Operations in Agda.
/// Consists of the operator, and the lhs and rhs of the expression.
pub struct BinOperator {pub symbol : Operator, pub lhs : Box<AgdaExpr>, pub rhs : Box<AgdaExpr>}