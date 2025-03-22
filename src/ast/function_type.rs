use crate::ast::agda_expr::AgdaExpr;

/// A type to denote functions in Agda.
/// Functions take the form lhs → rhs.
pub struct FunctionType {lhs : Box<AgdaExpr>, rhs : Box<AgdaExpr>}