use crate::ast::agda_expr::AgdaExpr;
use crate::ast::var_declaration::VarDecl;

/// A type to denote dependent functions in Agda.
/// These are of the form (a : A) (b : B) ... (n : N) → e.
/// Consists of a list of dependent variables, and the function body.
pub type DependentFunction = (Vec<VarDecl>, AgdaExpr);