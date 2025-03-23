use crate::ast::top_decl::TDeclaration;

/// Type to describe an Agda Program. Consists of a file name (String),
/// and a list of Declarations.
#[derive(PartialEq)]
pub struct Program {pub filepath : String, pub declarations : Vec<TDeclaration>}

#[macro_export]
macro_rules! program {
    ($filepath:expr, $decl:expr) => {
        Program {
            filepath: $filepath.to_string(),
            declarations: vec![($decl)]
        }
    }
}