use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Applications in Agda.
/// These are of the form a b, with a and b being Agda Expressions.
pub struct Application {pub lhs: Box<AgdaExpr>, pub rhs: Box<AgdaExpr>}