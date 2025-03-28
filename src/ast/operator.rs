
/// An enum to denote operators in Agda.
/// Consists of the symbol and its precedence.
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub enum Operator {
    Necessity = 9,
    Possibility = 8,
    PropEq = 0,
    Product = 1,
}
