use crate::ast::top_decl::TDeclaration;

/// Type to describe an Agda Program. Consists of a file name (String),
/// and a list of Declarations.
pub struct Program {pub file_name : String, pub declarations : Vec<TDeclaration>}
