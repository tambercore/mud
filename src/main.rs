mod ccg;
mod brill;
mod lambda;

use std::io::{Write};
use lambda::types::*;
use lambda::reduce::*;

fn main() {
    // Example expression: (λ"arg". arg) applied to "y"
    // Define your expression
    // Church-encoded TRUE: λt. λf. t
    let church_true = LambdaEntity::Abstraction(
        "t".to_string(),
        Box::new(LambdaEntity::Abstraction(
            "f".to_string(),
            Box::new(LambdaEntity::Variable("t".to_string())),
        )),
    );

    // Church-encoded FALSE: λt. λf. f
    let church_false = LambdaEntity::Abstraction(
        "t".to_string(),
        Box::new(LambdaEntity::Abstraction(
            "f".to_string(),
            Box::new(LambdaEntity::Variable("f".to_string())),
        )),
    );

    // Church-encoded AND: λp. λq. p q FALSE
    let church_and = LambdaEntity::Abstraction(
        "p".to_string(),
        Box::new(LambdaEntity::Abstraction(
            "q".to_string(),
            Box::new(LambdaEntity::Application(
                Box::new(LambdaEntity::Application(
                    Box::new(LambdaEntity::Variable("p".to_string())),
                    Box::new(LambdaEntity::Variable("q".to_string())),
                )),
                Box::new(church_false.clone()),
            )),
        )),
    );

    // Applying AND to TRUE and TRUE (should hold true)
    let expr = LambdaEntity::Application(
        Box::new(LambdaEntity::Application(
            Box::new(church_and),
            Box::new(church_true.clone()),
        )),
        Box::new(church_true),
    );



    println!("Expression: {}", expr);

    let expr2 = reduce(&expr);
    println!("Expression Reduced {}", expr2)

}