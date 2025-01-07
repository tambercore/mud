use std::any::Any;
use std::cmp::PartialEq;
use std::fmt;
use super::lambda_element::LambdaElement;

pub trait Substitutable {
    /// Substitute all free occurrences of `var` with `replacement`.
    fn substitute(&self, source: &LambdaEntity, target: &LambdaEntity) -> Self;
}

#[derive(Clone, Debug)]
pub enum LambdaEntity {
    Application(Box<LambdaEntity>, Box<LambdaEntity>),      // Application of two expressions
    Abstraction(Box<LambdaEntity>, Box<LambdaEntity>),                 // Lambda abstraction, e.g., λx.x + 1
    Variable(Box<LambdaElement>),                                       // Variable, e.g., x
}



// Implement Display for LambdaEntity to allow custom formatting
impl fmt::Display for LambdaEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LambdaEntity::Application(lhs, rhs) => write!(f, "({} {})", lhs, rhs),
            LambdaEntity::Abstraction(param, body) => {
                write!(f, "(λ{}. {})", param, body)
            }
            LambdaEntity::Variable(name) => write!(f, "{}", name),
        }
    }
}


impl PartialEq for LambdaEntity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LambdaEntity::Variable(v1), LambdaEntity::Variable(v2)) => *v1 == *v2,
            (LambdaEntity::Abstraction(var1, body1), LambdaEntity::Abstraction(var2, body2)) => {
                var1 == var2 && body1 == body2
            }
            (LambdaEntity::Application(left1, right1), LambdaEntity::Application(left2, right2)) => {
                left1 == left2 && right1 == right2
            }
            _ => false,
        }
    }
}