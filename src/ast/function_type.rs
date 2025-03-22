use crate::ast::agda_expr::AgdaExpr;

/// A type to denote functions in Agda.
/// Functions take the form lhs → rhs.
pub type FunctionType = (AgdaExpr, AgdaExpr);