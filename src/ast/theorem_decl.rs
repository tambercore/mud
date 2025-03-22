use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Theorems in Agda.
/// Consists of an identifier, hypothesis (type signature), proof (body), and an optional comment.
pub type Theorem = (String, AgdaExpr, AgdaExpr, Option<String>);