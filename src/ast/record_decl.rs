use crate::ast::var_declaration::VarDecl;

/// A type to denote Records in Agda.
/// Consists of Record name, Constructor name, a list of record fields, and an optional comment.
pub type Record = (String, String, Vec<VarDecl>, Option<String>);