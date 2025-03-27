use crate::ast::agda_expr::{format_agda_type, AgdaExpr};

/// A type to denote Theorems in Agda.
/// Consists of an identifier, hypothesis (type signature), proof (body), and an optional comment.

#[derive(Clone)]
pub struct Theorem {
    pub iden : String,
    pub hypothesis : Box<AgdaExpr>,
    pub proof : Box<AgdaExpr>,
    pub comment : Option<String>}

pub trait Agdaify {
    fn agdaify(&self) -> String;
}

impl Agdaify for Theorem {
    /// Converts the Theorem into valid Agda code.
     fn agdaify(&self) -> String {
        let mut code = String::new();
        code.push_str(&(format!("{} : {}\n", self.iden, format_agda_type(&self.hypothesis))));
        code.push_str(&(format!("{} = {}\n", self.iden, format_agda_type(&self.proof))));
        code
    }
}
impl PartialEq for Theorem {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden
    }}

#[macro_export]
macro_rules! theorem {
    ($iden:expr, $hypothesis:expr, $proof:expr, $comment:expr) => {
        TDeclaration::TheoremDecl(
        Theorem {iden: String::from($iden), hypothesis: Box::new($hypothesis), proof: Box::new($proof), comment: $comment})
    };
}