use crate::ast::var_declaration::VarDecl;

/// A type to denote Records in Agda.
/// Consists of Record name, Constructor name, a list of record fields, and an optional comment.
///
#[derive(PartialEq)]
pub struct Record {
    pub record_iden : String,
    pub constructor_iden : String,
    pub fields : Vec<VarDecl>,
    pub comment : Option<String>}

#[macro_export]
macro_rules! record {
    ($record_iden:expr, $constructor_iden:expr, $($field:expr),*) => {
        Record {
            record_iden: $record_iden.to_string(),
            constructor_iden: $constructor_iden.to_string(),
            fields: vec![$($field),*],
            comment: None
        }
    };
}