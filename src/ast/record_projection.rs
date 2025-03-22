use crate::ast::agda_expr::AgdaExpr;

/// A type to denote record projection in Agda.
/// These are of the form a.b, with a and b being Agda Expressions.
pub struct RecordProjection {lhs : Box<AgdaExpr>, rhs : Box<AgdaExpr>}