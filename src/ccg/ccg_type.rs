use std::fmt;
use serde::{Deserialize, Deserializer};
use crate::ccg::ccg_type_parser::parse_category;
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
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(parse_category(value.as_str()).unwrap().1)
    }
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
            CCGType::ConjunctionTag => {write!(f, "Conjunction Tag")}
        }
    }
}
