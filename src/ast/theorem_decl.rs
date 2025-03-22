use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Theorems in Agda.
/// Consists of an identifier, hypothesis (type signature), proof (body), and an optional comment.
pub struct Theorem {
    iden : String,
    hypothesis : Box<AgdaExpr>,
    proof : Box<AgdaExpr>,
    comment : Option<String>}