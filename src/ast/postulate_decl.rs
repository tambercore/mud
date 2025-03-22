use crate::ast::var_declaration::VarDecl;

/// A type to denote postulate in Agda.
/// Consists of a list of fields and an optional Comment.
pub struct Postulate {fields : Vec<VarDecl>, comment : Option<String>}