use warp::multipart::Part;
use crate::ast::agda_expr::AgdaExpr;

/// A type to denote variable declarations in Agda.
/// These take the form e : t where e is an identifier and t is an AgdaExpr.

#[derive(Clone)]
pub struct VarDecl {pub iden : String, pub _type : Box<AgdaExpr>}

impl PartialEq for VarDecl {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden && *self._type == *other._type
    }
}

#[macro_export]
macro_rules! var_decl {
    ($iden:expr, $type:expr) => {
        Box::from(VarDecl { iden: String::from($iden), _type: Box::from($type) })
    };
}