
/// An enum to denote operators in Agda.
/// Consists of the symbol and its precedence.
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Necessity = 9,
    Possibility = 8,
}
