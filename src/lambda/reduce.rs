use std::cmp::PartialEq;
use crate::lambda::types::*;
use crate::lambda::types::LambdaEntity::*;
use crate::montague::expression::Expression;
use crate::montague::expression::Expression::{Conjunction, Var};

fn _substitute(expression: &LambdaEntity, source: &LambdaEntity, target: &LambdaEntity) -> LambdaEntity {
    match expression {
        // If the node is an `Application`, recursively substitute on both sides
        Application(left, right) => {
            let left_substituted = _substitute(left, source, target);
            let right_substituted = _substitute(right, source, target);
            reduce(&Application(Box::new(left_substituted), Box::new(right_substituted)))
        }

        // If the variable in the abstraction matches `source`, return it unchanged; otherwise, substitute in the body
        Abstraction(variable, subexpr) => {
            if *variable == Box::from(source.clone()) {
                Abstraction(variable.clone(), subexpr.clone())
            } else {
                let subexpr_substituted = _substitute(subexpr, source, target);
                Abstraction(variable.clone(), Box::new(subexpr_substituted))
            }
        }

        // Handle substitution in variables or expressions
        Variable(variable) => {
            // Substitute variables and their content
            match *variable.clone() {
                Var(inner_var) => {
                    if Variable(Box::from(Var(String::from(inner_var)))) == source.clone() {
                        target.clone()
                    } else {
                        Variable(variable.clone())
                    }
                }
                Expression::Predicate(name, args) => {

                    // Substitute within predicate arguments
                    let substituted_args: Vec<_> = args
                        .iter()
                        .map(|arg| if Variable(Box::from(Var(String::from(arg)))) == source.clone() { target.to_string() } else { arg.clone() })
                        .collect();

                    let mut substituted_name = name.clone();

                    if name == source.clone() {
                        substituted_name = target.clone();
                    }

                    Variable(Box::from(Expression::Predicate(substituted_name, substituted_args)))
                }
                Conjunction(lhs, rhs) => {

                    println!("found conjunction with lhs {:?} and rhs {:?}. source: {:?}. target: {:?}", lhs, rhs, source, target);

                    // Substitute within both sides of a conjunction
                    let lhs_substituted = _substitute(
                        &*lhs,
                        source,
                        target,
                    );
                    let rhs_substituted = _substitute(
                        &*rhs,
                        source,
                        target,
                    );

                    println!("lhs substituted: {:?} rhs substituted: {:?}", lhs_substituted, rhs_substituted);

                    Variable(Box::new(Conjunction(Box::from(lhs_substituted), Box::from(rhs_substituted))))
                }

                Expression::ExistentialQuantifier(var, expr) => {

                    println!("found existential quantifier with var {} and expr {:?}. source: {:?}. target: {:?}", var, expr, source, target);

                    // If the bound variable matches `source`, skip substitution
                    if Variable(Box::from(Var(String::from(var.clone())))) == source.clone() {
                        LambdaEntity::Variable(Box::new(Expression::ExistentialQuantifier(
                            var.clone(),
                            expr.clone(),
                        )))
                    } else {
                        // Substitute within the inner expression
                        let substituted_expr = _substitute(
                            &*expr,
                            source,
                            target,
                        );

                        LambdaEntity::Variable(Box::new(Expression::ExistentialQuantifier(
                            var.clone(),
                            Box::from(substituted_expr.clone()),
                        )))

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
