use crate::ast::var_declaration::VarDecl;

/// A type to denote Records in Agda.
/// Consists of Record name, Constructor name, a list of record fields, and an optional comment.
pub struct Record {
    pub record_iden : String,
    pub constructor_iden : String,
    pub fields : Vec<VarDecl>,
    pub comment : Option<String>}