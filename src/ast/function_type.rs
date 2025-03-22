use crate::ast::agda_expr::AgdaExpr;

/// A type to denote functions in Agda.
/// Functions take the form lhs → rhs.
pub struct FunctionType {pub lhs : Box<AgdaExpr>, pub rhs : Box<AgdaExpr>}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}