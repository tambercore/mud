use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Applications in Agda.
/// These are of the form a b, with a and b being Agda Expressions.
pub type Application = (AgdaExpr, AgdaExpr);