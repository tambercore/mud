use crate::ast::operator::Operator;

/// A type to denote infix declarations of the form:
/// infix <int> <symbol>.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Infix {pub op : Operator}

#[macro_export]
macro_rules! infix {
    ($op:expr) => {
        TDeclaration::InfixDecl(Infix { op: $op })
    };
}