use std::any::Any;
use std::fmt;



#[derive(Clone, Debug)]
pub enum LambdaEntity {
    Application(Box<LambdaEntity>, Box<LambdaEntity>),      // Application of two expressions
    Abstraction(String, Box<LambdaEntity>),                 // Lambda abstraction, e.g., λx.x + 1
    Variable(String),                                       // Variable, e.g., x
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