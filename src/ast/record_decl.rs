use crate::ast::var_declaration::VarDecl;

/// A type to denote Records in Agda.
/// Consists of Record name, Constructor name, a list of record fields, and an optional comment.
pub struct Record {
    record_iden : String,
    constructor_iden : String,
    fields : Vec<VarDecl>,
    comment : Option<String>}