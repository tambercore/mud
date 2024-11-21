use std::fmt;
use crate::montague::expression::Expression::Predicate;

type Variable = String;
pub enum Expression {
    Conjunction(Box<Expression>, Box<Expression>),
    ExistentialQuantifier(Variable, Box<Expression>),
    Variable(Variable),
    Predicate(Variable, Vec<Variable>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Conjunction(lhs, rhs) => write!(f, "({} ∧ {})", lhs, rhs),
            Expression::ExistentialQuantifier(var, expr) => write!(f, "∃{}.{}", var, expr),
            Expression::Variable(name) => write!(f, "{}", name),
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

#[test]
fn test_expression_printing() {
    let expr = Expression::ExistentialQuantifier("x".to_string() ,
    Box::new(Expression::Conjunction(
        Box::new(Predicate("cheese".to_string(), vec!["x".to_string()])),
        Box::new(Predicate("likes".to_string(), vec!["John".to_string(), "x".to_string()])
    ))));

    println!("{}", expr)
}