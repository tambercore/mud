use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Theorems in Agda.
/// Consists of an identifier, hypothesis (type signature), proof (body), and an optional comment.
pub struct Theorem {
    pub iden : String,
    pub hypothesis : Box<AgdaExpr>,
    pub proof : Box<AgdaExpr>,
    pub comment : Option<String>}