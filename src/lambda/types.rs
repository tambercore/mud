use std::any::Any;
use std::fmt;


pub type LambdaVariable = (String, Box<dyn Any>);

pub enum LambdaEntity {
    Functor(Box<LambdaEntity>, Box<LambdaEntity>),       // Application of two expressions
    Abstraction(LambdaVariable, Box<LambdaEntity>),        // Lambda abstraction, e.g., λx.M
    Variable(LambdaVariable),                              // Variable, e.g., x
}


// Implement Display for LambdaEntity to allow custom formatting
impl fmt::Display for LambdaEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LambdaEntity::Functor(lhs, rhs) => write!(f, "({} {})", lhs, rhs),
            LambdaEntity::Abstraction(param, body) => {
                write!(f, "(λ{}. {})", param.0, body)
            }
            LambdaEntity::Variable(name) => write!(f, "{}", name.0),
        }
    }
}