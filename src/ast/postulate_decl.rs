use crate::ast::top_decl::TDeclaration;
use crate::ast::var_declaration::VarDecl;

/// A type to denote postulate in Agda.
/// Consists of a list of fields and an optional Comment.
#[derive(PartialEq, Clone)]
pub struct Postulate {pub fields : Vec<TDeclaration>, pub comment : Option<String>}

#[macro_export]
macro_rules! postulate {
    ($fields:expr, $comment:expr) => {
        TDeclaration::PostulateDecl(Postulate {fields: $fields, comment: $comment})
    };
}