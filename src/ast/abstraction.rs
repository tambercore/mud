use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Abstractions in Agda.
/// These are of the form λ x → y, where x is an identifier and y is an expression.
pub type Abstraction = (String, AgdaExpr);