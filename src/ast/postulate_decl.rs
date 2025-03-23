use crate::ast::var_declaration::VarDecl;

/// A type to denote postulate in Agda.
/// Consists of a list of fields and an optional Comment.
#[derive(PartialEq)]
pub struct Postulate {pub fields : Vec<VarDecl>, pub comment : Option<String>}

#[macro_export]
macro_rules! postulate {
    ($fields:expr, $comment:expr) => {
        Postulate {fields: $fields, comment: $comment}
    };
}