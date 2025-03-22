use crate::ast::operator::Operator;

/// A type to denote infix declarations of the form:
/// infix <int> <symbol>.
#[derive(Clone, Debug, PartialEq)]
pub struct Infix {pub op : Operator}