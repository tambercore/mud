use crate::ast::agda_expr::AgdaExpr;
use crate::ast::var_declaration::VarDecl;

/// A type to denote dependent functions in Agda.
/// These are of the form (a : A) (b : B) ... (n : N) → e.
/// Consists of a list of dependent variables, and the function body.
pub struct DependentFunction {pub vars : Vec<VarDecl>, pub expr : Box<AgdaExpr>}

impl PartialEq for DependentFunction {
    fn eq(&self, other: &Self) -> bool {
        self.vars == other.vars && self.expr == other.expr
    }
}