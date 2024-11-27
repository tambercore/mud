use crate::lambda::types::*;
use crate::lambda::types::LambdaEntity::{Application, Abstraction};
use crate::montague::expression::Expression;
use crate::montague::expression::Expression::Var;

fn _substitute(expression: &LambdaEntity, source: &str, target: &LambdaEntity) -> LambdaEntity {
    match expression {
        // If the node is an `Application`, recursively substitute on both sides
        LambdaEntity::Application(left, right) => {
            let left_substituted = _substitute(left, source, target);
            let right_substituted = _substitute(right, source, target);
            LambdaEntity::Application(Box::new(left_substituted), Box::new(right_substituted))
        }

        // If the variable in the abstraction matches `source`, return it unchanged; otherwise, substitute in the body
        LambdaEntity::Abstraction(variable, subexpr) => {
            if *variable == source {
                LambdaEntity::Abstraction(variable.clone(), subexpr.clone())
            } else {
                let subexpr_substituted = _substitute(subexpr, source, target);
                LambdaEntity::Abstraction(variable.clone(), Box::new(subexpr_substituted))
            }
        }

        // Handle substitution in variables or expressions
        LambdaEntity::Variable(variable) => {
            // Substitute variables and their content
            match *variable.clone() {
                Expression::Var(inner_var) => {
                    if inner_var == source {
                        target.clone()
                    } else {
                        LambdaEntity::Variable(variable.clone())
                    }
                }
                Expression::Predicate(name, args) => {

                    // Substitute within predicate arguments
                    let substituted_args: Vec<_> = args
                        .iter()
                        .map(|arg| if arg == source { target.to_string() } else { arg.clone() })
                        .collect();

                    let mut substituted_name = name.clone();

                    if source == name {
                        substituted_name = target.to_string();
                    }

                    LambdaEntity::Variable(Box::from(Expression::Predicate(substituted_name, substituted_args)))
                }
                Expression::Conjunction(lhs, rhs) => {

                    println!("found conjunction with lhs {:?} and rhs {:?}. source: {:?}. target: {:?}", lhs, rhs, source, target);

                    // Substitute within both sides of a conjunction
                    let lhs_substituted = _substitute(
                        &LambdaEntity::Variable(Box::new(*lhs.clone())),
                        source,
                        target,
                    );
                    let rhs_substituted = _substitute(
                        &LambdaEntity::Variable(Box::new(*rhs.clone())),
                        source,
                        target,
                    );

                    println!("lhs substituted: {:?} rhs substituted: {:?}", lhs_substituted, rhs_substituted);

                    // Ensure both are Expression::Variable before creating a Conjunction
                    if let LambdaEntity::Variable(boxed_lhs) = lhs_substituted {
                        if let LambdaEntity::Variable(boxed_rhs) = rhs_substituted {
                                    LambdaEntity::Variable(Box::new(Expression::Conjunction(
                                        boxed_lhs,
                                        boxed_rhs,
                                    )))
                                } else {
                                    panic!("Right-hand side substitution did not result in Expression::Variable");
                                }
                            } else {
                                panic!("Left-hand side substitution did not result in Expression::Variable");
                            }
                }

                Expression::ExistentialQuantifier(var, expr) => {

                    println!("found existential quantifier with var {} and expr {:?}. source: {:?}. target: {:?}", var, expr, source, target);

                    // If the bound variable matches `source`, skip substitution
                    if var == source {
                        LambdaEntity::Variable(Box::new(Expression::ExistentialQuantifier(
                            var.clone(),
                            expr.clone(),
                        )))
                    } else {
                        // Substitute within the inner expression
                        let substituted_expr = _substitute(
                            &LambdaEntity::Variable(Box::new(*expr.clone())),
                            source,
                            target,
                        );

                        // Ensure the substitution result is a valid expression before wrapping it back
                        if let LambdaEntity::Variable(substituted_box) = substituted_expr {
                            LambdaEntity::Variable(Box::new(Expression::ExistentialQuantifier(
                                var.clone(),
                                substituted_box,
                            )))
                        } else {
                            panic!("Substitution did not result in a valid LambdaEntity::Variable");
                        }
                    }
                }

            }
        }
    }
}




/// Function to reduce a lambda expression using a normal-order reduction strategy, i.e.
/// leftmost, outermost reduction. This uses the recursive `substitute` func defined above.
pub fn reduce(expression: &LambdaEntity) -> LambdaEntity {
    let expr2 = match expression {
        Application(expr, term) => {
            // Reduce the function (expr) and argument (term) before applying substitution.
            let reduced_expr = reduce(expr);
            let reduced_term = reduce(term);

            // Pattern match on `reduced_expr` directly, without dereferencing.
            if let Abstraction(var, body) = reduced_expr {
                let substituted_body = _substitute(&body, &var, &reduced_term);
                reduce(&substituted_body)
            } else {
                // If `reduced_expr` is not an abstraction, return the application as is.
                Application(Box::new(reduced_expr), Box::new(reduced_term))
            }
        }
        // If the expression is not an application, return it as is.
        _ => expression.clone(),
    };
    println!("reduced expr: {:?}", expr2);
    expr2
}
