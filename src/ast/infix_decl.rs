use crate::ast::operator::Operator;

/// A type to denote infix declarations of the form:
/// infix <int> <symbol>.
#[derive(Clone, Debug, PartialEq)]
pub struct Infix {pub op : Operator}

#[macro_export]
macro_rules! infix {
    ($op:expr) => {
        TDeclaration::InfixDecl(Infix { op: $op })
    };
}