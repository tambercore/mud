use crate::ast::operator::Operator;

/// A type to denote infix declarations of the form:
/// infix <int> <symbol>.
pub struct Infix {pub op : Operator}