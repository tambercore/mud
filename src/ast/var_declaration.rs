use warp::multipart::Part;
use crate::ast::agda_expr::{format_agda_type, AgdaExpr};

/// A type to denote variable declarations in Agda.
/// These take the form e : t where e is an identifier and t is an AgdaExpr.

#[derive(Debug, Clone)]
pub struct VarDecl {pub iden : String, pub _type : Box<AgdaExpr>}

impl PartialEq for VarDecl {
    fn eq(&self, other: &Self) -> bool {
        self.iden == other.iden && *self._type == *other._type
    }
}

impl VarDecl {

    pub fn agdaify(&self) -> String  {
        let mut code = String::new();
        let typ_str = format_agda_type(&*self._type);
        code.push_str(&format!("{} : {}\n", self.iden, typ_str));
        code
    }
}
#[macro_export]
macro_rules! var_decl {
    ($iden:expr, $type:expr) => {
        VarDecl { iden: String::from($iden), _type: Box::from($type) }
    };
}