mod ccg;
mod brill;
mod lambda;

use std::io::{Write};
use lambda::types::*;

fn main() {
    // Example expression: (λ"arg". arg) applied to "y"
    let expr = LambdaEntity::Functor(
        Box::new(LambdaEntity::Abstraction(
            ("x".to_string(), Box::new(1 as i32)),  // Using i32 as an example Any type
            Box::new(LambdaEntity::Variable(("x+1".to_string(), Box::new(1 as i32)))),
        )),
        Box::new(LambdaEntity::Variable(("5".to_string(), Box::new(1 as i32)))),
    );

    println!("Expression: {}", expr);
}