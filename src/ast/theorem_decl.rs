use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Theorems in Agda.
/// Consists of an identifier, hypothesis (type signature), proof (body), and an optional comment.

pub struct Theorem {
    pub iden : String,
    pub hypothesis : Box<AgdaExpr>,
    pub proof : Box<AgdaExpr>,
    pub comment : Option<String>}

impl PartialEq for Theorem {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden
    }}

#[macro_export]
macro_rules! theorem {
    ($iden:expr, $hypothesis:expr, $proof:expr, $comment:expr) => {
        Theorem {iden: $iden, hypothesis: Box::new($hypothesis), proof: Box::new($proof), comment: $comment}
    };
}