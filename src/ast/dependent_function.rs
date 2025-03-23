use crate::ast::agda_expr::AgdaExpr;
use crate::ast::var_declaration::VarDecl;

/// A type to denote dependent functions in Agda.
/// These are of the form (a : A) (b : B) ... (n : N) → e.
/// Consists of a list of dependent variables, and the function body.
#[derive(Clone)]
pub struct DependentFunction {pub bound_var: VarDecl, pub expr : Box<AgdaExpr>}

impl PartialEq for DependentFunction {
    fn eq(&self, other: &Self) -> bool {
        self.bound_var == other.bound_var && self.expr == other.expr
    }
}

#[macro_export]
macro_rules! dependent_function {
    ($vars: expr, $expr: expr) => {
        DependentFunction {
            bound_var: $vars,
            expr: Box::new($expr)
        }
    };
}