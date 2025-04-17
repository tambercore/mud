use crate::ast::agda_expr::AgdaExpr;

/// A type to denote Abstractions in Agda.
///
/// An abstraction in Agda is represented by the lambda notation `λ x → y`, where:
/// - `x` is the variable (identifier) bound in the abstraction, and
/// - `y` is the expression that follows the lambda.
///
/// This struct holds the variable and the expression as part of an abstraction.
#[derive(Eq, Hash, Debug, Clone)]
pub struct Abstraction
{pub var: String, pub expr: Box<AgdaExpr>}



/// Compares two abstractions for equality.
///
/// Two abstractions are considered equal if both the variable (`var`) and the expression (`expr`)
/// are equal.
impl PartialEq for Abstraction {
    fn eq(&self, other: &Self) -> bool {
        self.var == other.var && self.expr == other.expr
    }
}



/// A macro to conveniently create an `Abstraction` wrapped in an `AgdaExpr::Abs` variant.
#[macro_export]
macro_rules! abstraction {
    ($var:expr, $expr:expr) => {
        AgdaExpr::Abs( Abstraction { var: $var.to_string(), expr: Box::new($expr) })
    };
}