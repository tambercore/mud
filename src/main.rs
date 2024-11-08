mod ccg;
mod brill;
mod lambda;

use std::io::{Write};
use lambda::types::*;
use lambda::reduce::*;

fn main() {
    // Example expression: (λ"arg". arg) applied to "y"
    let expr = LambdaEntity::Application(
        Box::new(LambdaEntity::Abstraction(
            "x".to_string(),  // Using i32 as an example Any type
            Box::new(LambdaEntity::Variable(("x".to_string()))),
        )),
        Box::new(LambdaEntity::Variable(("5".to_string()))),
    );

    println!("Expression: {}", expr);

    let expr2 = reduce(&expr);
    println!("Expression Reduced {}", expr2)

}