use crate::ast::agda_expr::AgdaExpr;
use crate::ast::var_declaration::VarDecl;

/// A type to denote Quantification in Agda.
/// Consists of the quantifier symbol, a list of dependent variables and an expression body.
pub struct Quantification {symbol : String, vars : Vec<VarDecl>, expr : Box<AgdaExpr>}