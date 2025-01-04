use std::fmt;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use crate::ccg::ccg_type::CCGType::*;
use crate::ccg::ccg_types::{CCGCategory, CCGOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum CCGType {
    ForwardsFunctor(Box<CCGType>, Box<CCGType>),
    BackwardsFunctor(Box<CCGType>, Box<CCGType>),
    Conjunction,
    ConjunctionTag,
    Noun,
    NounPhrase,
    PrepositionalPhrase,
    Punctuation,
    Sentence,
    Empty
}



/// Implementation for `Deserialize` to parse CCG categories from JSON strings.
impl<'de> Deserialize<'de> for CCGType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, try to deserialize the value as a simple string.
        let value: String = Deserialize::deserialize(deserializer)?;

        // Match the string to the known simple category types.
        match value.as_str() {
            "s" => Ok(Sentence),
            "np" => Ok(NounPhrase),
            "n" => Ok(Noun),
            "conj" => Ok(Conjunction),
            "[conj]" => Ok(ConjunctionTag),
            "p" => Ok(PrepositionalPhrase),
            "punc" => Ok(Punctuation),
            "" => Ok(Empty),
            _ => { // Complex category
                // If it's not a simple category, attempt to parse it as a composed category.
                if let Some((left_str, op, right_str)) = parse_composed_category(value.as_str()) {
                    let left = Box::new(Self::from_string(left_str).map_err(D::Error::custom)?);
                    let operator = match op {
                        "/" => CCGOperator::Forward,
                        "\\" => CCGOperator::Backward,
                        _ => return Err(D::Error::custom(format!("Invalid composition operator: {}", value))),
                    };
                    let right = Box::new(Self::from_string(right_str).map_err(D::Error::custom)?);

                    // Return the composed category.
                    Ok(CCGCategory::Composed { left, right, operator })
                } else {
                    // If the category is invalid, return an error.
                    Err(D::Error::custom(format!("Invalid category format: {}", value.as_str())))
                }
            }
        }
    }
}



/// Helper function to parse composed categories from a string.
/// A composed category is in the form of "(left_category operator right_category)".
fn parse_composed_category(value: &str) -> Option<(&str, &str, &str)> {
    // If the value is a simple category like (s) or (s\np), remove the outer parentheses
    let value = if is_simple_parentheses(value) {
        &value[1..value.len() - 1]  // Slice to remove the first and last character
    } else {
        value
    };

    if let Some(pos) = find_operator_position_outside_parentheses(value) {
        let (left, remainder) = value.split_at(pos);
        let (operator, right) = remainder.split_at(1);
        Some((left.trim(), operator, right.trim()))
    } else {
        None
    }
}


/// Checks if the string is a simple category surrounded by parentheses (e.g., (s) or (s\np))
fn is_simple_parentheses(value: &str) -> bool {
    // A simple parenthesized category should not contain any operators and should not be empty
    if value.starts_with('(') && value.ends_with(')') {
        let inner = &value[1..value.len() - 1]; // Remove the outer parentheses
        !inner.contains('(') && !inner.contains(')')
    } else {
        false
    }
}


/// Finds the position of an operator (`/` or `\`) that is outside of parentheses.
/// The operato r is considered outside of parentheses if the string starts with `(`
/// and ends with `)` and there is a matching parenthesis pair around the operator.
fn find_operator_position_outside_parentheses(value: &str) -> Option<usize> {
    let mut paren_count = 0;

    // Iterate over each character in the string
    for (i, c) in value.char_indices() {
        match c {
            '(' => paren_count += 1,   // Increment count on encountering '('
            ')' => paren_count -= 1,   // Decrement count on encountering ')'
            // If it's an operator and we are outside parentheses
            '/' | '\\' if paren_count == 0 => return Some(i),
            _ => {}
        }
    }
    None
}




impl fmt::Display for CCGType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CCGType::ForwardsFunctor(left, right) => write!(f, "({} / {})", left, right),
            CCGType::BackwardsFunctor(left, right) => write!(f, "({} \\ {})", left, right),
            CCGType::Conjunction => write!(f, "Conjunction"),
            CCGType::ConjunctionTag => write!(f, "ConjunctionTag"),
            CCGType::Noun => write!(f, "Noun"),
            CCGType::NounPhrase => write!(f, "Noun Phrase"),
            CCGType::PrepositionalPhrase => write!(f, "Prepositional Phrase"),
            CCGType::Punctuation => write!(f, "Punctuation"),
            CCGType::Sentence => write!(f, "Sentence"),
            CCGType::Empty => write!(f, "Empty Type"),
            ConjunctionTag => {write!(f, "Conjunction Tag")}
        }
    }
}
