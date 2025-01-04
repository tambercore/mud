use std::fmt;
use crate::lambda::types::LambdaEntity;
use crate::lambda::types::LambdaEntity::*;
use crate::montague::expression::Expression::{Predicate, Var};

#[derive(Clone, Debug)]
pub enum Expression {
    Conjunction(Box<LambdaEntity>, Box<LambdaEntity>),
    ExistentialQuantifier(String, Box<LambdaEntity>),
    Var(String),
    Predicate(LambdaEntity, Vec<String>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Conjunction(lhs, rhs) => write!(f, "({} ∧ {})", lhs, rhs),
            Expression::ExistentialQuantifier(var, expr) => write!(f, "∃{}.{}", var, expr),
            Expression::Var(name) => write!(f, "{}", name),
            Expression::Predicate(name, args) => {
                if args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    let args_str = args.join(", ");
                    write!(f, "{}({})", name, args_str)
                }
            }
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Var(v1), Expression::Var(v2)) => v1 == v2,
            (Expression::Predicate(pred1, args1), Expression::Predicate(pred2, args2)) => {
                pred1 == pred2 && args1 == args2
            }
            _ => false,
        }
    }
}
#[test]
fn test_expression_printing() {
    let expr = Expression::ExistentialQuantifier("x".to_string() ,
                                                 Box::from(Variable(Box::new(Expression::Conjunction(
                                                     Box::new(Variable(Box::from(Predicate(Variable(Box::from(Var("cheese".to_string()))), vec!["x".to_string()])))),
                                                     Box::new(Variable(Box::from(Predicate(Variable(Box::from(Var("likes".to_string()))), vec!["John".to_string(), "x".to_string()]))))
                                                     )))));

    println!("{}", expr)
}