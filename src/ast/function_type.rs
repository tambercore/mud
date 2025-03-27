use crate::ast::agda_expr::AgdaExpr;

/// A type to denote functions in Agda.
/// Functions take the form lhs → rhs.
#[derive(Debug, Clone)]
pub struct FunctionType {pub lhs : Box<AgdaExpr>, pub rhs : Box<AgdaExpr>}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}

#[macro_export]
macro_rules! function_type {
    ($lhs:expr, $rhs:expr) => {
        AgdaExpr::FunType(
        FunctionType {
            lhs: Box::new($lhs),
            rhs: Box::new($rhs)
        })
    };
}