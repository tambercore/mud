use crate::ast::agda_expr::AgdaExpr;

/// A type to denote variable declarations in Agda.
/// These take the form e : t where e is an identifier and t is an AgdaExpr.
pub struct VarDecl {iden : String, _type : Box<AgdaExpr>}